use crate::{Planet, Ship};

// Average time step at 60 FPS
const TIME_STEP: f32 = 1000.0 / 60.0;

const G: f32 = 6.6743e-11;

// Returns orbital period according to
// T = 2π √(r³/GM)
// where
// T = orbital period
// r = orbit radius
// G = gravitational constant
// M = mass of central body
pub fn orbital_period(central_mass: f32, radius: f32) -> f32 {
    0.0001 * 2.0 * std::f32::consts::PI * (radius * radius * radius / (G * central_mass)).sqrt()
}

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
            planet.x = f32::cos(std::f32::consts::PI * 2.0 * time / planet.orbit_speed)
                * planet.orbit_radius;
            planet.y = f32::sin(std::f32::consts::PI * 2.0 * time / planet.orbit_speed)
                * planet.orbit_radius;
        }

        for planet in &planets {
            let dx = planet.x - ship.x;
            let dy = planet.y - ship.y;

            let d2 = dx * dx + dy * dy;

            let f = planet.mass / d2;

            ship.vx += f * dx * TIME_STEP;
            ship.vy += f * dy * TIME_STEP;
        }

        ship.x += ship.vx * TIME_STEP;
        ship.y += ship.vy * TIME_STEP;

        Some(TrajectoryStep {
            n,
            t: time,
            x: ship.x,
            y: ship.y,
        })
    })
}
