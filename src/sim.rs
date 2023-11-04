use crate::prelude::*;

pub fn tic(game: &Game, player: &mut Ship, planets: &mut [Planet]) {
    for step in 0..game.steps() {
        eval(game.time + (step as f32) * DT, player, planets);
    }
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

            if dist >= planet.radius * 2.0 + 1000.0 {
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

fn eval(time: f32, player: &mut Ship, planets: &mut [Planet]) {
    for planet_id in 0..planets.len() {
        let parent_pos = planets[planet_id]
            .parent
            .map(|parent_id| planets[parent_id].pos)
            .unwrap_or_default();

        let planet = &mut planets[planet_id];

        let orbit =
            0.05 * PI * 2.0 * time / planet.orbit_speed + planet.orbit_offset;

        let orbit = vec2(orbit.cos(), orbit.sin());

        planet.pos = parent_pos + orbit * planet.orbit_radius;
    }

    // ---

    for planet in planets.iter() {
        let d = planet.pos - player.pos;
        let f = planet.mass / d.length_squared();

        player.vel += f * d * DT;
    }

    player.pos += player.vel * DT;
}
