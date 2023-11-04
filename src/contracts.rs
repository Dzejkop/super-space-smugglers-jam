use crate::prelude::arrow::Arrow;
use crate::prelude::*;

pub struct Contract {
    pub planet: usize,
    pub destination: usize,
    pub cargo: Cargo,
    pub reward: u32,
}

pub enum Cargo {
    Passengers,
}

pub fn tic(
    camera: &Camera,
    game: &mut Game,
    player: &Ship,
    planets: &[Planet],
) {
    let mo = mouse();

    for contract in &game.contracts {
        let planet = &planets[contract.planet];
        let planet_pos = camera.world_to_screen(planet.pos);

        if (time() / 1000.0) as i32 % 2 == 0 {
            Img::sprite_idx_with_size(320, uvec2(2, 2))
                .at(planet_pos + vec2(64.0, -64.0) * camera.zoom)
                .scale(4.0 * camera.zoom)
                .draw();
        }

        let mouse_pos = camera.screen_to_world(vec2(mo.x as f32, mo.y as f32));
        let cursor_to_planet_distance = (planet.pos - mouse_pos).length();

        if cursor_to_planet_distance < planet.radius {
            SelectionIndicator::new(planet_pos)
                .size(Vec2::ONE * planet.radius * 1.2)
                .draw();

            if mouse_left_pressed() {

            }

        }
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
    }
}
