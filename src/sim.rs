use crate::contracts::MIN_ACCEPT_DISTANCE;
use crate::prelude::*;

pub fn tic(game: &Game, player: &mut Player, planets: &mut [Planet]) {
    for step in 0..game.steps() {
        eval(game.time + (step as f32) * DT, &mut player.ship, planets);
    }
}

#[derive(Clone, Copy)]
pub struct TrajectoryStep {
    pub pos: Vec2,
    pub color: u8,
    pub touches: bool,
}

pub fn trajectory(
    game: &Game,
    player: &Ship,
    planets: &[Planet],
) -> impl Iterator<Item = TrajectoryStep> {
    let mut step = 0;
    let mut time = game.time;
    let mut player = *player;
    let mut planets = planets.to_vec();

    let mut prev_pos = vec2(f32::MAX, f32::MAX);

    std::iter::from_fn(move || {
        loop {
            step += 1;
            time += DT;

            if step > 650 {
                return None;
            }

            sim::eval(time, &mut player, &mut planets);

            if player.pos.distance(prev_pos) >= 100.0 {
                break;
            }
        }

        let mut closest_color = 12;
        let mut closest_dist = f32::MAX;
        let mut touches = false;

        for planet in &planets {
            let dist = planet.pos.distance(player.pos);

            if dist < closest_dist {
                closest_color = planet.color;
                closest_dist = dist;
            }

            if dist <= planet.radius + MIN_ACCEPT_DISTANCE {
                touches = true;
                break;
            }
        }

        prev_pos = player.pos;

        Some(TrajectoryStep {
            pos: player.pos,
            color: closest_color,
            touches,
        })
    })
}

pub fn eval(time: f32, player: &mut Ship, planets: &mut [Planet]) {
    for planet_id in 0..planets.len() {
        let parent_pos = planets[planet_id]
            .parent
            .map(|parent_id| planets[parent_id].pos)
            .unwrap_or_default();

        let planet = &mut planets[planet_id];
        let orbit = PI * 2.0 * time / planet.orbit_speed + planet.orbit_phase;
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
