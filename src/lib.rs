#![feature(const_fn_floating_point_arithmetic)]
#![feature(extract_if)]

#[macro_use]
mod tic80;

mod alloc;
mod camera;
mod game;
mod intro;
mod msgs;
mod orbits;
mod overflow_indicator;
mod particles;
mod planet;
mod planets;
mod player;
mod police;
mod ship;
mod text;
mod ui;
mod utils;

mod prelude {
    pub(crate) use std::f32::consts::PI;

    pub(crate) use glam::*;
    pub(crate) use rand::prelude::*;

    pub(crate) use crate::camera::Camera;
    pub(crate) use crate::game::{Game, GameSpeed};
    pub(crate) use crate::overflow_indicator::OverflowIndicator;
    pub(crate) use crate::planet::Planet;
    pub(crate) use crate::ship::{Ship, ShipSprite};
    pub(crate) use crate::text::Text;
    pub(crate) use crate::tic80::*;
    pub(crate) use crate::utils::*;
    pub(crate) use crate::{msgs, orbits, particles, police};
}

use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::prelude::*;

static mut RNG: Option<SmallRng> = None;

enum State {
    Intro,
    Playing,
    GameOver,
}

// TODO change before release
static mut STATE: State = State::Playing;

#[export_name = "TIC"]
pub fn tic() {
    let rng = unsafe { RNG.get_or_insert_with(|| SmallRng::seed_from_u64(64)) };
    let state = unsafe { &mut STATE };

    cls(0);

    match state {
        State::Intro => {
            if intro::tic() {
                *rng = SmallRng::seed_from_u64(time().to_bits() as u64);
                *state = State::Playing;
            }

            particles::tic(rng, None);
        }

        State::Playing | State::GameOver => unsafe {
            game::tic();
            camera::tic();
            planets::tic(camera::get(), game::get());
            player::tic(camera::get(), game::get(), planets::get());

            if police::tic(
                rng,
                camera::get(),
                player::get(),
                planets::get(),
                game::get_mut(),
            ) {
                *state = State::GameOver;
            }

            draw_space_and_stuff(
                camera::get(),
                game::get_mut(),
                player::get_mut(),
                planets::get(),
            );

            particles::tic(rng, Some(camera::get()));
            msgs::tic();
            ui::tic(game::get_mut(), police::get());
        },
    }
}

fn draw_space_and_stuff(
    camera: &Camera,
    game: &mut Game,
    player: &mut Ship,
    planets: &[Planet],
) {
    let mo = mouse();

    let m = camera.screen_to_world(vec2(mo.x as f32, mo.y as f32));
    let d = (player.pos - m).length_squared();

    let s = camera.world_to_screen(player.pos);
    let m = camera.world_to_screen(m);

    let dv = (s - m) * 0.01;

    if mo.left
        && game.is_paused()
        && d < (30.0 / camera.zoom)
        && !game.manouver_mode
    {
        game.manouver_mode = true;
    }

    if game.manouver_mode && !mo.left {
        game.manouver_mode = false;

        player.vel += dv;
    }

    if game.manouver_mode {
        let mut t_ship = *player;

        t_ship.vel += dv;

        line(s.x, s.y, m.x, m.y, 12);

        let mut prev_step = t_ship.pos;

        for step in orbits::trajectory(game.time(), &t_ship, planets).take(1000)
        {
            let step = vec2(step.x, step.y);
            let p1 = camera.world_to_screen(prev_step);
            let p2 = camera.world_to_screen(step);

            line(p1.x, p1.y, p2.x, p2.y, 12);

            prev_step = step;
        }
    }
}
