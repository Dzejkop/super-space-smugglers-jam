mod alloc;
mod camera;
mod intro;
mod orbits;
mod particles;
mod ship;
mod text;
mod tic80;
mod utils;

use self::camera::*;
use self::tic80::sys::print;
use self::tic80::*;
use self::utils::*;
use crate::orbits::simulate_trajectory;
use crate::ship::*;
use crate::text::*;

use glam::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;

#[derive(Clone)]
pub struct Ship {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

static mut SHIP: Ship = Ship {
    x: 175.0,
    y: 0.0,
    vx: 0.0,
    vy: -1.0,
};

#[derive(Clone)]
pub struct Planet {
    x: f32,
    y: f32,

    // Oribital characteristics
    orbit_radius: f32,
    orbit_speed: f32,

    radius: f32,
    mass: f32,
    color: u8,
}

static mut PLANETS: [Planet; 3] = [
    // The Sun
    Planet {
        x: 0.0,
        y: 0.0,
        orbit_radius: 0.0,
        orbit_speed: 0.0,
        radius: 100.0,
        mass: 1.0,
        color: 4,
    },
    Planet {
        x: 0.0,
        y: 0.0,
        orbit_radius: 250.0,
        orbit_speed: 0.0002,
        radius: 5.0,
        mass: 0.1,
        color: 3,
    },
    Planet {
        x: 0.0,
        y: 0.0,
        orbit_radius: 500.0,
        orbit_speed: 0.00001,
        radius: 2.0,
        mass: 0.01,
        color: 2,
    },
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

#[export_name = "TIC"]
pub fn tic() {
    let rng = unsafe { RNG.get_or_insert_with(|| SmallRng::seed_from_u64(64)) };
    let state = unsafe { &mut STATE };

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

fn draw_space_and_stuff() {
    let m = mouse();

    if m.left && !unsafe { MOUSE_LEFT_PREV } {
        unsafe {
            SHIP.x = m.x as f32;
            SHIP.y = m.y as f32;

            SHIP.vx = 0.0;
            SHIP.vy = 1.0;
        }
    }

    // Update the camera
    let camera = camera_mut();

    const CAMERA_SPEED: f32 = 2.0;

    if key(keys::A) {
        camera.x += CAMERA_SPEED / camera.remap_zoom();
    }

    if key(keys::D) {
        camera.x -= CAMERA_SPEED / camera.remap_zoom();
    }

    if key(keys::W) {
        camera.y += CAMERA_SPEED / camera.remap_zoom();
    }

    if key(keys::S) {
        camera.y -= CAMERA_SPEED / camera.remap_zoom();
    }

    let target_zoom = if m.scroll_y > 0 {
        1.0
    } else if m.scroll_y < 0 {
        0.0
    } else {
        camera.zoom
    };

    let zoom_change = (target_zoom - camera.zoom) * ZOOM_LERP;

    camera.zoom += zoom_change;

    // Draw the planets
    unsafe {
        let (ox, oy) = camera.world_to_screen_integer(0.0, 0.0);

        for planet in &mut PLANETS.iter_mut() {
            planet.x = f32::sin(time() * planet.orbit_speed) * planet.orbit_radius;
            planet.y = f32::cos(time() * planet.orbit_speed) * planet.orbit_radius;

            // Draw orbit
            circb(
                ox,
                oy,
                (planet.orbit_radius * camera.remap_zoom()) as i32,
                planet.color,
            );

            // Draw planet
            let (x, y) = camera.world_to_screen_integer(planet.x, planet.y);
            circ(
                x,
                y,
                (camera.remap_zoom() * planet.radius) as i32,
                planet.color,
            );
        }
    }

    // Draw the ship
    unsafe {
        for planet in &PLANETS {
            let dx = planet.x - SHIP.x;
            let dy = planet.y - SHIP.y;
            let d2 = dx * dx + dy * dy;

            let f = planet.mass / d2;

            SHIP.vx += f * dx;
            SHIP.vy += f * dy;
        }

        SHIP.x += SHIP.vx;
        SHIP.y += SHIP.vy;

        let (x, y) = camera.world_to_screen_integer(SHIP.x, SHIP.y);

        Img::sprite_idx_with_size(258, uvec2(2, 2))
            .scale(camera.remap_zoom())
            .at(vec2(x as f32, y as f32))
            .draw();

        let mut prev_step = [SHIP.x, SHIP.y];
        for (idx, step) in simulate_trajectory(time(), &SHIP, &PLANETS)
            .iter()
            .enumerate()
        {
            if idx % 100 == 0 {
                let (x1, y1) = camera.world_to_screen_integer(prev_step[0], prev_step[1]);
                let (x2, y2) = camera.world_to_screen_integer(step.x, step.y);

                line(x1 as f32, y1 as f32, x2 as f32, y2 as f32, 12);

                prev_step = [step.x, step.y];
            }
        }
    }

    unsafe {
        MOUSE_LEFT_PREV = m.left;
    }

    unsafe {
        let t_delta = time() - TIME_PREV;
        let time_as_text = t_delta.to_string();

        print(time_as_text.as_bytes().as_ptr(), 0, 0, 7, true, 1, false);

        TIME_PREV = time();
    }
}
