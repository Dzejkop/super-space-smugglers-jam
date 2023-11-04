use crate::prelude::*;

pub fn tic(game: &Game, ship: &mut Ship, planets: &mut [Planet]) {
    for _ in 0..game.steps() {
        for planet_id in 0..planets.len() {
            let planet;
            let parent;

            if let Some(parent_id) = planets[planet_id].parent {
                let [a, b] =
                    planets.get_many_mut([planet_id, parent_id]).unwrap();

                planet = a;
                parent = Some(b);
            } else {
                planet = &mut planets[planet_id];
                parent = None;
            }

            if let Some(parent) = parent {
                planet.pos.x = parent.pos.x
                    + f32::cos(PI * 2.0 * game.time / planet.orbit_speed)
                        * planet.orbit_radius;

                planet.pos.y = parent.pos.y
                    + f32::sin(PI * 2.0 * game.time / planet.orbit_speed)
                        * planet.orbit_radius;
            } else {
                planet.pos.x =
                    f32::cos(PI * 2.0 * game.time / planet.orbit_speed)
                        * planet.orbit_radius;

                planet.pos.y =
                    f32::sin(PI * 2.0 * game.time / planet.orbit_speed)
                        * planet.orbit_radius;
            }
        }

        // ---

        for planet in planets.iter() {
            let d = planet.pos - ship.pos;
            let d2 = d.length_squared();
            let f = planet.mass / d2;

            ship.vel += f * d * DT;
        }

        ship.pos += ship.vel * DT;
    }
}
