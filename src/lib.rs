#![feature(const_fn_floating_point_arithmetic)]

mod alloc;
mod camera;
mod game_state;
mod intro;
mod orbits;
mod particles;
mod planet;
mod ship;
mod sprites;
mod text;
mod tic80;
mod utils;

use std::f32::consts::PI;

use glam::*;
use orbits::orbital_period;
use planet::Planet;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use self::camera::*;
use self::game_state::*;
use self::text::*;
use self::tic80::sys::print;
use self::tic80::*;
use self::utils::*;
use crate::game_state::game_mut;
use crate::orbits::simulate_trajectory;
use crate::ship::*;

#[derive(Clone)]
pub struct Ship {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,

    // Index of planet
    in_orbit: Option<usize>,
}

static mut SHIP: Ship = Ship {
    x: 175.0,
    y: 0.0,
    vx: 0.0,
    vy: 1.0,
    in_orbit: None,
};

static mut PLANETS: [Planet; 4] = [
    // 0: The Sun
    Planet::base()
        .with_radius(120.0)
        .with_mass(0.5)
        .with_color(4),
    // 1: Mercury-ish
    Planet::base()
        .with_orbit(1000.0, 0.0002)
        .with_radius(30.0)
        .with_mass(0.1)
        .with_color(3),
    // 2: Venus-ish
    Planet::base()
        .with_orbit(2000.0, 0.00001)
        .with_radius(30.0)
        .with_mass(0.1)
        .with_color(4),
    // 3: Jupiter-ish
    Planet::base()
        .with_orbit(4000.0, 0.0002)
        .with_radius(60.0)
        .with_mass(0.22)
        .with_color(6),
    // 4: Europa
    // Planet::planet(0.0, 0.0, 2000.0, 0.00001, 30.0, 0.1, 2),
    // Planet::base()
    //     .with_orbit(300.0, 0.002)
    //     .with_radius(15.0)
    //     .with_mass(0.01)
    //     .with_color(9)
    //     .moon_of(3),
];

static mut MOUSE_LEFT_PREV: bool = false;
static mut TIME_PREV: f32 = 0.0;

static mut RNG: Option<SmallRng> = None;

enum State {
    Intro,
    Playing,
}

// TODO change before release
static mut STATE: State = State::Playing;

static mut INIT: bool = false;

#[export_name = "TIC"]
pub fn tic() {
    let rng = unsafe { RNG.get_or_insert_with(|| SmallRng::seed_from_u64(64)) };
    let state = unsafe { &mut STATE };

    if !unsafe { INIT } {
        init();

        unsafe {
            INIT = true;
        }
    }

    // ---

    cls(0);

    match state {
        State::Intro => {
            if intro::tic() {
                *state = State::Playing;
            }

            particles::tic(rng);
        }

        State::Playing => {
            draw_space_and_stuff();
        }
    }
}

fn init() {
    let central_mass = unsafe { PLANETS[0].mass };

    for planet in unsafe { &mut PLANETS } {
        planet.orbit_speed = orbital_period(central_mass, planet.radius);
    }
}

fn draw_space_and_stuff() {
    let m = mouse();

    // Update the camera
    let camera = camera_mut();
    let game = game_mut();

    game.update();
    let dt = game.dt();

    const CAMERA_SPEED: f32 = 2.0;

    if key(keys::A) {
        camera.pos.x += CAMERA_SPEED / camera.zoom;
    }

    if key(keys::D) {
        camera.pos.x -= CAMERA_SPEED / camera.zoom;
    }

    if key(keys::W) {
        camera.pos.y += CAMERA_SPEED / camera.zoom;
    }

    if key(keys::S) {
        camera.pos.y -= CAMERA_SPEED / camera.zoom;
    }

    if m.scroll_y != 0 {
        let world_pos = camera.screen_to_world(mouse().x as i32, mouse().y as i32);
        let screen_pos_before = camera.world_to_screen(world_pos.0, world_pos.1);

        if m.scroll_y > 0 {
            camera.zoom *= 1.2;
        } else {
            camera.zoom /= 1.2;
        }

        camera.zoom = camera.zoom.clamp(MIN_ZOOM, MAX_ZOOM);

        let screen_pos_after = camera.world_to_screen(world_pos.0, world_pos.1);

        camera.pos.x -= (screen_pos_after.0 - screen_pos_before.0) / camera.zoom;
        camera.pos.y -= (screen_pos_after.1 - screen_pos_before.1) / camera.zoom;
    }

    // Draw the planets
    unsafe {
        let (ox, oy) = camera.world_to_screen_integer(0.0, 0.0);

        for planet in &mut PLANETS.iter_mut() {
            if let Some(parent) = planet.parent {
                let parent = &PLANETS[parent];

                planet.x = parent.x
                    + f32::cos(std::f32::consts::PI * 2.0 * game.time() / planet.orbit_speed)
                        * planet.orbit_radius;
                planet.y = parent.y
                    + f32::sin(std::f32::consts::PI * 2.0 * game.time() / planet.orbit_speed)
                        * planet.orbit_radius;

                let (ox, oy) = camera.world_to_screen_integer(parent.x, parent.y);
                // Draw orbit
                circb(
                    ox,
                    oy,
                    (planet.orbit_radius * camera.zoom) as i32,
                    planet.color,
                );
            } else {
                planet.x = f32::cos(std::f32::consts::PI * 2.0 * game.time() / planet.orbit_speed)
                    * planet.orbit_radius;
                planet.y = f32::sin(std::f32::consts::PI * 2.0 * game.time() / planet.orbit_speed)
                    * planet.orbit_radius;

                // Draw orbit
                circb(
                    ox,
                    oy,
                    (planet.orbit_radius * camera.zoom) as i32,
                    planet.color,
                );
            }

            // Draw planet
            let (x, y) = camera.world_to_screen_integer(planet.x, planet.y);
            circ(x, y, (camera.zoom * planet.radius) as i32, planet.color);
        }
    }

    // Draw the ship
    unsafe {
        if dt != 0.0 {
            for planet in &PLANETS {
                let dx = planet.x - SHIP.x;
                let dy = planet.y - SHIP.y;

                let d2 = dx * dx + dy * dy;

                let f = planet.mass / d2;

                SHIP.vx += f * dx * dt;
                SHIP.vy += f * dy * dt;
            }

            SHIP.x += SHIP.vx * dt;
            SHIP.y += SHIP.vy * dt;
        }

        let (x, y) = camera.world_to_screen_integer(SHIP.x, SHIP.y);

        ShipSprite::player()
            .at(vec2(x as f32, y as f32))
            .rot(game.time() * 0.001)
            .scale((3.0 * camera.zoom).max(0.15))
            .engine(true)
            .draw();

        if camera.zoom < 0.2 {
            if time() % 1000.0 < 500.0 {
                circb(x, y, 8, 5);
            }
        }

        if (x < 4 || y < 4 || x > WIDTH + 4 || y > HEIGHT + 4) && time() % 1000.0 < 500.0 {
            let pos_a = vec2(x as f32, y as f32);

            let pos_b = pos_a.clamp(
                vec2(2.0, 2.0),
                vec2(WIDTH as f32 - 2.0, HEIGHT as f32 - 2.0),
            );

            let dir = (pos_a - pos_b).normalize();

            let arrow = [
                (vec2(0.0, 8.0), vec2(0.0, 1.0)),
                (vec2(0.0, 2.0), vec2(-3.0, 4.0)),
                (vec2(0.0, 2.0), vec2(3.0, 4.0)),
            ];

            let transform =
                |v: Vec2| -> Vec2 { rotate(pos_b + v, pos_b, PI - dir.angle_between(Vec2::Y)) };

            for (v0, v1) in arrow {
                let v0 = transform(v0);
                let v1 = transform(v1);

                line(v0.x, v0.y, v1.x, v1.y, 5);
            }
        }
    }

    unsafe {
        let (mx, my) = camera.screen_to_world(m.x as i32, m.y as i32);

        let dx = SHIP.x - mx;
        let dy = SHIP.y - my;

        let d = dx * dx + dy * dy;

        let (sx, sy) = camera.world_to_screen(SHIP.x, SHIP.y);
        let (mx, my) = camera.world_to_screen(mx, my);

        let dvx = (sx - mx) * 0.01;
        let dvy = (sy - my) * 0.01;

        if m.left && game.is_paused() && d < (30.0 / camera.zoom) && !game.manouver_mode {
            game.manouver_mode = true;
        }

        if game.manouver_mode && !m.left {
            game.manouver_mode = false;

            SHIP.vx += dvx;
            SHIP.vy += dvy;
        }

        if game.manouver_mode {
            let mut t_ship = SHIP.clone();

            t_ship.vx += dvx;
            t_ship.vy += dvy;

            line(sx, sy, mx, my, 12);

            let mut prev_step = [t_ship.x, t_ship.y];
            for step in simulate_trajectory(game.time(), &t_ship, &PLANETS).take(1000) {
                let (x1, y1) = camera.world_to_screen_integer(prev_step[0], prev_step[1]);
                let (x2, y2) = camera.world_to_screen_integer(step.x, step.y);

                line(x1 as f32, y1 as f32, x2 as f32, y2 as f32, 12);

                prev_step = [step.x, step.y];
            }
        }
    }

    draw_ui(game);

    unsafe {
        MOUSE_LEFT_PREV = m.left;
    }
}

fn draw_ui(game: &mut Game) {
    let m = mouse();
    let mx = m.x as i32;
    let my = m.y as i32;

    let mouse_over_stop_button = mx >= WIDTH - (16 * 3) - 4
        && mx < WIDTH - (16 * 4) - 4 + 16 * 2
        && my >= HEIGHT - 16 - 4
        && my < HEIGHT - 16 - 4 + 16 * 2;

    let mouse_over_play_button = mx >= WIDTH - (16 * 2) - 4
        && mx < WIDTH - (16 * 3) - 4 + 16 * 2
        && my >= HEIGHT - 16 - 4
        && my < HEIGHT - 16 - 4 + 16 * 2;

    let mouse_over_fast_button = mx >= WIDTH - (16 * 1) - 4
        && mx < WIDTH - (16 * 2) - 4 + 16 * 2
        && my >= HEIGHT - 16 - 4
        && my < HEIGHT - 16 - 4 + 16 * 2;

    spr(
        if matches!(game.speed, GameSpeed::Stop) {
            sprites::ui::buttons::active::STOP
        } else if mouse_over_stop_button {
            sprites::ui::buttons::highlighted::STOP
        } else {
            sprites::ui::buttons::inactive::STOP
        },
        WIDTH - (16 * 3) - 4,
        HEIGHT - 16 - 4,
        SpriteOptions {
            w: 2,
            h: 2,
            transparent: &[0],
            ..Default::default()
        },
    );

    spr(
        if matches!(game.speed, GameSpeed::Normal) {
            sprites::ui::buttons::active::NORMAL
        } else if mouse_over_play_button {
            sprites::ui::buttons::highlighted::NORMAL
        } else {
            sprites::ui::buttons::inactive::NORMAL
        },
        WIDTH - (16 * 2) - 4,
        HEIGHT - 16 - 4,
        SpriteOptions {
            w: 2,
            h: 2,
            transparent: &[0],
            ..Default::default()
        },
    );

    spr(
        if matches!(game.speed, GameSpeed::Fast) {
            sprites::ui::buttons::active::FAST
        } else if mouse_over_fast_button {
            sprites::ui::buttons::highlighted::FAST
        } else {
            sprites::ui::buttons::inactive::FAST
        },
        WIDTH - (16 * 1) - 4,
        HEIGHT - 16 - 4,
        SpriteOptions {
            w: 2,
            h: 2,
            transparent: &[0],
            ..Default::default()
        },
    );

    Text::new(format!("Day: {}", game.day()))
        .at(vec2(0.0, 0.0))
        .draw();

    Text::new(format!("Fuel: {}%", game.fuel()))
        .at(vec2(0.0, 8.0))
        .draw();

    Text::new(format!("Money: {}", game.money_str()))
        .at(vec2(0.0, 16.0))
        .draw();

    Text::new(format!("Tickets: {}", game.tickets()))
        .at(vec2(0.0, 24.0))
        .draw();

    // ---

    if m.left && !unsafe { MOUSE_LEFT_PREV } {
        if mouse_over_stop_button {
            game.speed = GameSpeed::Stop;
        } else if mouse_over_play_button {
            game.speed = GameSpeed::Normal;
        } else if mouse_over_fast_button {
            game.speed = GameSpeed::Fast;
        }
    }

    if key(keys::DIGIT_1) {
        game.speed = GameSpeed::Stop;
    } else if key(keys::DIGIT_2) {
        game.speed = GameSpeed::Normal;
    } else if key(keys::DIGIT_3) {
        game.speed = GameSpeed::Fast;
    } else if keyp(keys::SPACE, 16, 16) {
        game.speed = match game.speed {
            GameSpeed::Stop => GameSpeed::Normal,
            _ => GameSpeed::Stop,
        };
    }
}
