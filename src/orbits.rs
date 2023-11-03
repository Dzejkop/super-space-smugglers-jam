use crate::{Planet, Ship};

// Average time step at 60 FPS
const TIME_STEP: f32 = 1000.0 / 60.0;

#[derive(Clone, Copy)]
pub struct TrajectoryStep {
    pub n: usize,
    pub t: f32,
    pub x: f32,
    pub y: f32,
}

pub fn simulate_trajectory<const N: usize>(
    t: f32,
    ship: &Ship,
    planets: &[Planet; N],
) -> impl Iterator<Item = TrajectoryStep> {
    let mut planets: [Planet; N] = planets.clone();
    let mut ship = ship.clone();
    let mut n = 0;

    std::iter::from_fn(move || {
        n += 1;

        let time = t + TIME_STEP * n as f32;

        for planet in &mut planets {
            planet.x = f32::sin(time * planet.orbit_speed) * planet.orbit_radius;
            planet.y = f32::cos(time * planet.orbit_speed) * planet.orbit_radius;
        }

        for planet in &planets {
            let dx = planet.x - ship.x;
            let dy = planet.y - ship.y;

            let d = dx * dx + dy * dy;

            let f = planet.mass / d;

            ship.vx += f * dx;
            ship.vy += f * dy;
        }

        ship.x += ship.vx;
        ship.y += ship.vy;

        Some(TrajectoryStep {
            n,
            t: time,
            x: ship.x,
            y: ship.y,
        })
    })
}
