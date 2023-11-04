#![feature(const_fn_floating_point_arithmetic)]
#![feature(extract_if)]

#[macro_use]
mod tic80;

mod alloc;
mod camera;
mod contracts;
mod game;
mod intro;
mod manouvers;
mod mouse_utils;
mod msgs;
mod orbits;
mod overflow_indicator;
mod particles;
mod planet;
mod planets;
mod player;
mod police;
mod selection_indicator;
mod ship;
mod text;
mod ui;
mod utils;

mod prelude {
    pub(crate) use std::f32::consts::PI;

    pub(crate) use glam::*;
    pub(crate) use rand::prelude::*;

    pub(crate) use crate::camera::Camera;
    pub(crate) use crate::contracts::{Cargo, Contract};
    pub(crate) use crate::game::{Game, GameSpeed};
    pub(crate) use crate::mouse_utils::mouse_left_pressed;
    pub(crate) use crate::overflow_indicator::OverflowIndicator;
    pub(crate) use crate::planet::Planet;
    pub(crate) use crate::selection_indicator::SelectionIndicator;
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

            manouvers::tic(
                camera::get(),
                game::get_mut(),
                player::get_mut(),
                planets::get(),
            );

            contracts::tic(
                camera::get(),
                game::get_mut(),
                player::get_mut(),
                planets::get(),
            );

            particles::tic(rng, Some(camera::get()));
            msgs::tic(game::get());
            ui::tic(game::get_mut(), police::get());
            overflow_indicator::tic();

            mouse_utils::tic();
        },
    }
}
