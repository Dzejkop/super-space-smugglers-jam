#![feature(extract_if)]

#[macro_use]
mod tic80;

mod alloc;
mod camera;
mod contracts;
mod fuel;
mod game;
mod intro;
mod manouvers;
mod mouse_utils;
mod msgs;
mod overflow_indicator;
mod particles;
mod planet;
mod planets;
mod player;
mod police;
mod selection_indicator;
mod ship;
mod sim;
mod text;
mod ui;
mod utils;

mod prelude {
    pub(crate) use std::f32::consts::PI;

    pub(crate) use glam::*;
    pub(crate) use rand::prelude::*;

    pub(crate) use crate::arrow::Arrow;
    pub(crate) use crate::camera::Camera;
    pub(crate) use crate::contracts::Cargo;
    pub(crate) use crate::game::{Game, GameSpeed};
    pub(crate) use crate::mouse_utils::{
        mouse_left_pressed, mouse_right_pressed,
    };
    pub(crate) use crate::overflow_indicator::OverflowIndicator;
    pub(crate) use crate::planet::Planet;
    pub(crate) use crate::player::Player;
    pub(crate) use crate::selection_indicator::SelectionIndicator;
    pub(crate) use crate::ship::{Ship, ShipSprite};
    pub(crate) use crate::text::Text;
    pub(crate) use crate::tic80::*;
    pub(crate) use crate::utils::*;
    pub(crate) use crate::{msgs, particles, police, sim};
}

use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::prelude::*;

static mut RNG: Option<SmallRng> = None;

enum State {
    Intro,
    Spawning,
    Playing,
    GameOver,
}

// TODO change before release
static mut STATE: State = State::Spawning;

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

            particles::tic(rng, None, None);
        }

        State::Spawning | State::Playing | State::GameOver => unsafe {
            if game::get().time == 0.0 {
                planets::init(planets::galaxies::gamma());
            }

            if let State::Spawning | State::Playing = state {
                game::tic();
                camera::tic();
                particles::tic(rng, Some(game::get()), Some(camera::get()));
            }

            planets::tic(camera::get());

            if player::tic(camera::get(), game::get()) {
                *state = State::Playing;
            }

            if police::tic(
                rng,
                camera::get(),
                player::get(),
                planets::get(),
                game::get_mut(),
            ) {
                *state = State::GameOver;
                player::get_mut().is_caught = true;
            }

            if let State::Playing = state {
                manouvers::tic(
                    camera::get(),
                    game::get_mut(),
                    player::get_mut(),
                    planets::get(),
                );

                contracts::tic(
                    rng,
                    camera::get(),
                    game::get_mut(),
                    player::get_mut(),
                    planets::get(),
                    police::get_mut(),
                );

                fuel::tic(
                    camera::get(),
                    game::get_mut(),
                    player::get_mut(),
                    planets::get(),
                );

                msgs::tic(game::get());
                overflow_indicator::tic();
                ui::tic(game::get_mut(), police::get());
                sim::tic(game::get(), player::get_mut(), planets::get_mut());
            }

            if let State::GameOver = state {
                let y = 22.0;

                Text::new("Ouch, you've been caught!")
                    .at(vec2(WIDTH as f32, y + 16.0))
                    .align_center()
                    .draw();

                Text::new("Your criminal days are over.")
                    .at(vec2(WIDTH as f32, y + 24.0))
                    .align_center()
                    .draw();

                Text::new(format!(
                    "You've tasted freedom for {} days.",
                    game::get().day()
                ))
                .at(vec2(WIDTH as f32, y + 50.0))
                .align_center()
                .color(4)
                .draw();

                Text::new(format!(
                    "You've collected ${}k credits.",
                    game::get().day()
                ))
                .at(vec2(WIDTH as f32, y + 58.0))
                .align_center()
                .color(4)
                .draw();

                ShipSprite::police()
                    .at(vec2(18.0, y + 22.0))
                    .rot((time() / 333.0).sin())
                    .draw(None);

                ShipSprite::police()
                    .at(vec2(WIDTH as f32 - 18.0, y + 22.0))
                    .rot((time() / 333.0).cos())
                    .draw(None);

                Text::new("Use escape to restart the game")
                    .at(vec2(WIDTH as f32, HEIGHT as f32 - 20.0))
                    .color(5)
                    .align_center()
                    .draw();

                camera::get_mut().zoom /= 1.0025;
            }

            mouse_utils::tic();
        },
    }
}
