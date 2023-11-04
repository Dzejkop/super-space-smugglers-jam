#![feature(const_fn_floating_point_arithmetic)]

#[macro_use]
mod tic80;

mod alloc;
mod camera;
mod game;
mod intro;
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
    pub(crate) use crate::{orbits, particles};
}

use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::prelude::*;

static mut RNG: Option<SmallRng> = None;

enum State {
    Intro,
    Playing,
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

            particles::tic(rng);
        }

        State::Playing => unsafe {
            game::tic();
            camera::tic();
            planets::tic(camera::get(), game::get());
            player::tic(camera::get(), game::get(), planets::get());
            police::tic(rng, camera::get(), player::get(), game::get());

            draw_space_and_stuff(
                camera::get(),
                game::get_mut(),
                player::get(),
                planets::get(),
            );

            ui::tic(game::get_mut());
        },
    }
}

fn draw_space_and_stuff(
    camera: &Camera,
    game: &mut Game,
    player: &Ship,
    planets: &[Planet],
) {
    let m = mouse();

    unsafe {
        let (mx, my) = camera.screen_to_world(m.x as i32, m.y as i32);

        let dx = player.pos.x - mx;
        let dy = player.pos.y - my;

        let d = dx * dx + dy * dy;

        let (sx, sy) = camera.world_to_screen(player.pos.x, player.pos.y);
        let (mx, my) = camera.world_to_screen(mx, my);

        let dvx = (sx - mx) * 0.01;
        let dvy = (sy - my) * 0.01;

        if m.left
            && game.is_paused()
            && d < (30.0 / camera.zoom)
            && !game.manouver_mode
        {
            game.manouver_mode = true;
        }

        if game.manouver_mode && !m.left {
            game.manouver_mode = false;

            player::get_mut().vel.x += dvx;
            player::get_mut().vel.y += dvy;
        }

        if game.manouver_mode {
            let mut t_ship = *player::get();

            t_ship.vel.x += dvx;
            t_ship.vel.y += dvy;

            line(sx, sy, mx, my, 12);

            let mut prev_step = [t_ship.pos.x, t_ship.pos.y];

            for step in
                orbits::trajectory(game.time(), &t_ship, planets).take(1000)
            {
                let (x1, y1) =
                    camera.world_to_screen_integer(prev_step[0], prev_step[1]);

                let (x2, y2) = camera.world_to_screen_integer(step.x, step.y);

                line(x1 as f32, y1 as f32, x2 as f32, y2 as f32, 12);

                prev_step = [step.x, step.y];
            }
        }
    }
}
