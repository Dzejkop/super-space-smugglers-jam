use crate::prelude::*;

const G: f32 = 6.6743e-11;

// Returns orbital period according to
// T = 2π √(r³/GM)
// where
// T = orbital period
// r = orbit radius
// G = gravitational constant
// M = mass of central body
pub fn orbital_period(central_mass: f32, radius: f32) -> f32 {
    0.0001 * 2.0 * PI * (radius * radius * radius / (G * central_mass)).sqrt()
}

#[derive(Clone, Copy)]
pub struct TrajectoryStep {
    pub n: usize,
    pub t: f32,
    pub x: f32,
    pub y: f32,
}

pub fn trajectory(
    t: f32,
    ship: &Ship,
    planets: &[Planet],
) -> impl Iterator<Item = TrajectoryStep> {
    let mut planets = planets.to_vec();
    let mut ship = ship.clone();
    let mut n = 0;

    std::iter::from_fn(move || {
        n += 1;

        let time = t + DT * n as f32;

        for planet in &mut planets {
            planet.pos.x = f32::cos(PI * 2.0 * time / planet.orbit_speed)
                * planet.orbit_radius;

            planet.pos.y = f32::sin(PI * 2.0 * time / planet.orbit_speed)
                * planet.orbit_radius;
        }

        for planet in &planets {
            let d = planet.pos - ship.pos;
            let d2 = d.length_squared();
            let f = planet.mass / d2;

            ship.vel += f * d * DT;
        }

        ship.pos += ship.vel * DT;

        Some(TrajectoryStep {
            n,
            t: time,
            x: ship.pos.x,
            y: ship.pos.y,
        })
    })
}
