use crate::prelude::*;

pub fn tic(game: &Game, player: &mut Ship, planets: &mut [Planet]) {
    for step in 0..game.steps() {
        eval(game.time + (step as f32) * DT, player, planets);
    }
}

pub fn eval(time: f32, player: &mut Ship, planets: &mut [Planet]) {
    for planet_id in 0..planets.len() {
        let parent_pos = planets[planet_id]
            .parent
            .map(|parent_id| planets[parent_id].pos)
            .unwrap_or_default();

        let planet = &mut planets[planet_id];
        let orbit = PI * 2.0 * time / planet.orbit_speed;
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
