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
    pub x: f32,
    pub y: f32,
    pub c: u8,
}

pub fn trajectory(
    game: &Game,
    player: &Ship,
    planets: &[Planet],
) -> impl Iterator<Item = TrajectoryStep> {
    let mut step = 0;
    let mut time = game.time;
    let mut player = player.clone();
    let mut planets = planets.to_vec();

    std::iter::from_fn(move || {
        step += 1;
        time += DT;

        sim::eval(time, &mut player, &mut planets);

        let mut closest_color = 12;
        let mut closest_dist = f32::MAX;

        for planet in &planets {
            let dist = planet.pos.distance(player.pos);

            if dist >= planet.radius * 2.0 + 300.0 {
                continue;
            }

            if dist < closest_dist {
                closest_color = planet.color;
                closest_dist = dist;
            }
        }

        Some(TrajectoryStep {
            n: step,
            x: player.pos.x,
            y: player.pos.y,
            c: closest_color,
        })
    })
}
