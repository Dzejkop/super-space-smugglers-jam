use crate::police::PoliceState;
use crate::prelude::keys::P;
use crate::prelude::sprites::buttons;
use crate::prelude::*;

pub const MIN_ACCEPT_DISTANCE: f32 = 250.0;
pub const MIN_DELIVERY_DISTANCE: f32 = 250.0;

pub const MIN_DELAY_BETWEEN_NEW_CONTRACTS: f32 = 5000.0;
pub const MAX_CONTRACTS: usize = 3;

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
    rng: &mut dyn RngCore,
    camera: &Camera,
    game: &mut Game,
    player: &Player,
    planets: &[Planet],
    police: &mut PoliceState,
) {
    let mo = mouse();

    // Spawn new contracts

    if game.contracts.len() < MAX_CONTRACTS {
        if game.time - game.time_of_last_contract_spawned
            > MIN_DELAY_BETWEEN_NEW_CONTRACTS
        {
            if rng.gen::<f32>() > 0.5 {
                // Spawn new contract

                music(
                    tracks::NEW_CONTRACT_SOUND,
                    MusicOptions {
                        repeat: false,
                        ..Default::default()
                    },
                );

                msgs::add("New contract available!");

                game.time_of_last_contract_spawned = game.time;

                let planet = rng.gen_range(1..planets.len());
                let mut dest = rng.gen_range(1..planets.len());

                while dest == planet {
                    dest = rng.gen_range(1..planets.len());
                }

                let cargo = match rng.gen_range(0..3) {
                    0 => Cargo::Passengers,
                    1 => Cargo::Crabs,
                    2 => Cargo::Bananas,
                    _ => unreachable!(),
                };

                game.contracts.push(Contract {
                    planet,
                    destination: dest,
                    cargo,
                    reward: 20,
                });
            }
        }
    }

    // Draw available unselected contracts
    for (idx, contract) in game.contracts.iter().enumerate() {
        let planet = &planets[contract.planet];
        let planet_pos = camera.world_to_screen(planet.pos);

        let flash_indicator = (time() / 1000.0) as i32 % 2 == 0;

        if flash_indicator {
            Img::sprite_idx_with_size(320, uvec2(2, 2))
                .at(planet_pos + vec2(64.0, -64.0) * camera.zoom)
                .scale((4.0 * camera.zoom).max(0.3))
                .draw();
        }

        let mouse_pos = camera.screen_to_world(vec2(mo.x as f32, mo.y as f32));
        let cursor_to_planet_distance = (planet.pos - mouse_pos).length();
        let ship_to_planet_distance = (player.ship.pos - planet.pos).length();

        if camera.zoom < 0.15 && flash_indicator && !game.manouver_mode {
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

    // Draw selected contract ui
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
                music(
                    tracks::COIN_SOUND,
                    MusicOptions {
                        repeat: false,
                        ..Default::default()
                    },
                );
                game.contracts.remove(selected_contract);
                police.increment_wanted_level();
            }
        }

        Img::sprite_idx_with_size(sprite_idx as u32, uvec2(2, 2))
            .at(button_pos)
            .draw();
    }

    // If mouse is over cargo hold, show arrows to destinations
    let mpos = vec2(mo.x as f32, mo.y as f32);

    let cargo_hold_height = 3.0 * 16.0;

    let cargo_hold_bounds = (
        vec2(16.0, HEIGHT as f32 - cargo_hold_height),
        vec2(16.0, cargo_hold_height),
    );

    if mpos.x > cargo_hold_bounds.0.x
        && mpos.x < cargo_hold_bounds.0.x + cargo_hold_bounds.1.x
        && mpos.y > cargo_hold_bounds.0.y
        && mpos.y < cargo_hold_bounds.0.y + cargo_hold_bounds.1.y
    {
        for (idx, cargo) in game.cargo_hold.iter().enumerate() {
            if let Some(contract) = cargo {
                let planet = &planets[contract.destination];
                let planet_pos = camera.world_to_screen(planet.pos);

                let idx = 2 - idx;

                let cargo_hold_middle = cargo_hold_bounds.0
                    + vec2(cargo_hold_bounds.1.x / 2.0, 0.0)
                    + vec2(0.0, 8.0 + 16.0 * idx as f32);

                Arrow::new(cargo_hold_middle, planet_pos, planet.color)
                    .margin(5.0)
                    .draw();
            }
        }
    }

    // Deliveries
    let mut deliveries_to_clear = vec![];

    for (idx, contract) in game.cargo_hold.iter().enumerate() {
        if let Some(contract) = contract {
            let planet = &planets[contract.destination];

            let ship_to_planet_distance =
                (player.ship.pos - planet.pos).length();

            if ship_to_planet_distance < planet.radius + MIN_DELIVERY_DISTANCE {
                game.credits += contract.reward;
                game.total_credits += contract.reward;

                music(
                    tracks::COIN_SOUND,
                    MusicOptions {
                        repeat: false,
                        ..Default::default()
                    },
                );

                msgs::add("Delivery complete!");

                deliveries_to_clear.push(idx);
            }
        }
    }

    for idx in deliveries_to_clear {
        game.cargo_hold[idx] = None;
    }
}
