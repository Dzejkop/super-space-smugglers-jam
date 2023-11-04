use crate::mouse_utils::mouse_right_pressed;
use crate::prelude::arrow::Arrow;
use crate::prelude::sprites::buttons;
use crate::prelude::*;

pub const MIN_ACCEPT_DISTANCE: f32 = 100.0;

#[derive(Clone, Copy)]
pub struct Contract {
    pub planet: usize,
    pub destination: usize,
    pub cargo: Cargo,
    pub reward: u32,
}

#[derive(Clone, Copy)]
pub enum Cargo {
    Passengers,
    Crabs,
    Bananas,
}

impl Cargo {
    pub fn sprite(&self) -> u32 {
        match self {
            Cargo::Passengers => 352,
            Cargo::Bananas => 354,
            Cargo::Crabs => 356,
        }
    }
}

pub fn insert_into_empty_cargo<const N: usize>(
    contract: Contract,
    hold: &mut [Option<Contract>; N],
) -> bool {
    for n in 0..N {
        if hold[n].is_none() {
            hold[n] = Some(contract);
            return true;
        }
    }

    false
}

pub fn tic(
    camera: &Camera,
    game: &mut Game,
    player: &Ship,
    planets: &[Planet],
) {
    let mo = mouse();

    // Draw available unselected contracts
    for (idx, contract) in game.contracts.iter().enumerate() {
        let planet = &planets[contract.planet];
        let planet_pos = camera.world_to_screen(planet.pos);

        let flash_indicator = (time() / 1000.0) as i32 % 2 == 0;

        if flash_indicator {
            Img::sprite_idx_with_size(320, uvec2(2, 2))
                .at(planet_pos + vec2(64.0, -64.0) * camera.zoom)
                .scale(4.0 * camera.zoom)
                .draw();
        }

        let mouse_pos = camera.screen_to_world(vec2(mo.x as f32, mo.y as f32));
        let cursor_to_planet_distance = (planet.pos - mouse_pos).length();

        let ship_to_planet_distance = (player.pos - planet.pos).length();

        if camera.zoom < 0.15 && flash_indicator {
            circb(planet_pos.x as i32, planet_pos.y as i32, 8, 3);
        }

        if cursor_to_planet_distance < planet.radius
            && ship_to_planet_distance < planet.radius + MIN_ACCEPT_DISTANCE
        {
            SelectionIndicator::new(planet_pos)
                .size(Vec2::ONE * planet.radius * 1.2)
                .draw();

            if mouse_left_pressed() {
                game.selected_contract = Some(idx);

                // Enforce pause game, and disable manouver mode
                game.manouver_mode = false;
                game.speed = GameSpeed::Stop;
            }
        }
    }

    if game.manouver_mode || !game.is_paused() {
        game.selected_contract = None;
    }

    if game.selected_contract.is_some() && mouse_right_pressed() {
        game.selected_contract = None;
    }

    if let Some(selected_contract) = game.selected_contract {
        let contract = &game.contracts[selected_contract];

        let planet = &planets[contract.planet];
        let destination = &planets[contract.destination];

        let planet_pos = camera.world_to_screen(planet.pos);
        let destination_pos = camera.world_to_screen(destination.pos);

        Arrow::new(planet_pos, destination_pos, destination.color)
            .margin(5.0)
            .draw();

        let middle_point = (planet_pos + destination_pos) / 2.0;

        Text::new("Accept?")
            .at(middle_point + vec2(0.0, 8.0))
            .draw();

        let mpos = vec2(mo.x as f32, mo.y as f32);

        let button_pos = middle_point + vec2(0.0, 32.0);

        let mouse_over_accept_button = (mpos - button_pos).length() < 8.0;

        let sprite_idx = if mouse_over_accept_button {
            buttons::highlighted::OK
        } else {
            buttons::inactive::OK
        };

        if mouse_over_accept_button && mouse_left_pressed() {
            game.selected_contract = None;

            if !insert_into_empty_cargo(*contract, &mut game.cargo_hold) {
                msgs::add("Cargo hold is full!");
            } else {
                game.contracts.remove(selected_contract);
            }
        }

        Img::sprite_idx_with_size(sprite_idx as u32, uvec2(2, 2))
            .at(button_pos)
            .draw();
    }
}
