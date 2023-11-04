use crate::prelude::*;

const MIN_REFUEL_DISTANCE: f32 = 250.0;
const REFUELLING_PLANETS: &[usize] = &[7, 11];

pub fn tic(
    camera: &Camera,
    game: &mut Game,
    player: &Player,
    planets: &[Planet],
) {
    let mo = mouse();

    let mpos = vec2(mo.x as f32, mo.y as f32);

    let fuel_gauge_height = 48.0;
    let fuel_gauge_bounds = (
        vec2(0.0, HEIGHT as f32 - fuel_gauge_height),
        vec2(16.0, HEIGHT as f32),
    );

    let fuel_gauge_middle = fuel_gauge_bounds.0 + fuel_gauge_bounds.1 / 2.0;

    if mpos.x > fuel_gauge_bounds.0.x
        && mpos.x < fuel_gauge_bounds.0.x + fuel_gauge_bounds.1.x
        && mpos.y > fuel_gauge_bounds.0.y
        && mpos.y < fuel_gauge_bounds.0.y + fuel_gauge_bounds.1.y
    {
        for planet in REFUELLING_PLANETS {
            let planet = &planets[*planet];

            let planet_pos = camera.world_to_screen(planet.pos);

            Arrow::new(fuel_gauge_middle, planet_pos, planet.color)
                .margin(5.0)
                .draw();
        }
    }

    // Refuel
    if game.fuel >= 1.0 {
        return;
    }

    for planet in REFUELLING_PLANETS {
        let planet = &planets[*planet];

        let distance_to_player = (player.ship.pos - planet.pos).length();

        if distance_to_player < planet.radius + MIN_REFUEL_DISTANCE {
            game.fuel = game.fuel.max(1.0);

            music(
                tracks::REFUEL_SOUND,
                MusicOptions {
                    repeat: false,
                    ..Default::default()
                },
            );

            msgs::add("Refuelled!");

            break;
        }
    }
}
