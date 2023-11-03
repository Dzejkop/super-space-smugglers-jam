mod alloc;
mod camera;
mod orbits;
mod particles;
mod tic80;
mod utils;

use self::camera::*;

use self::tic80::sys::print;
use self::tic80::*;
use self::utils::*;
use crate::orbits::simulate_trajectory;

mod sprites {}

mod btns {
    pub const UP: i32 = 0;
    pub const DOWN: i32 = 1;
    pub const LEFT: i32 = 2;
    pub const RIGHT: i32 = 3;
}

mod keys {
    pub const A: i32 = 1;
    pub const B: i32 = 2;
    pub const C: i32 = 3;
    pub const D: i32 = 4;
    pub const E: i32 = 5;
    pub const F: i32 = 6;
    pub const G: i32 = 7;
    pub const H: i32 = 8;
    pub const I: i32 = 9;
    pub const J: i32 = 10;
    pub const K: i32 = 11;
    pub const L: i32 = 12;
    pub const M: i32 = 13;
    pub const N: i32 = 14;
    pub const O: i32 = 15;
    pub const P: i32 = 16;
    pub const Q: i32 = 17;
    pub const R: i32 = 18;
    pub const S: i32 = 19;
    pub const T: i32 = 20;
    pub const U: i32 = 21;
    pub const V: i32 = 22;
    pub const W: i32 = 23;
    pub const X: i32 = 24;
    pub const Y: i32 = 25;
    pub const Z: i32 = 26;
}

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
    x: 0.0,
    y: 0.0,
    vx: 0.0,
    vy: 0.0,
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

#[export_name = "TIC"]
pub fn tic() {
    let rng = unsafe { RNG.get_or_insert_with(|| SmallRng::seed_from_u64(64)) };

    // ---

    cls(0);

    draw_ship(vec2(32.0, 32.0), time() * 0.001);
    draw_space_and_stuff();

    particles::tic(rng);
}

fn draw_ship(at: Vec2, rot: f32) {
    Img::sprite_xy(uvec2(17, 18), uvec2(2, 2))
        .at(at)
        .rot(rot)
        .draw();

    Img::sprite_xy(uvec2(17, 20), uvec2(2, 2))
        .at(rotate(at + vec2(0.0, 16.0), at, rot))
        .rot(rot)
        .draw();
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
    unsafe {
        if key(keys::A) {
            camera_mut().x += 1.0;
        }

        if key(keys::D) {
            camera_mut().x -= 1.0;
        }

        if key(keys::W) {
            camera_mut().y += 1.0;
        }

        if key(keys::S) {
            camera_mut().y -= 1.0;
        }

        camera_mut().zoom += m.scroll_y as f32 * 0.001;
    }

    // Draw the planets
    unsafe {
        let (ox, oy) = camera().world_to_screen(0.0, 0.0);

        for planet in &mut PLANETS.iter_mut() {
            planet.x = f32::sin(time() * planet.orbit_speed) * planet.orbit_radius;
            planet.y = f32::cos(time() * planet.orbit_speed) * planet.orbit_radius;

            // Draw orbit
            circb(
                ox,
                oy,
                (planet.orbit_radius * camera().zoom) as i32,
                planet.color,
            );

            // Draw planet
            let (x, y) = camera().world_to_screen(planet.x, planet.y);
            circ(x, y, (camera().zoom * planet.radius) as i32, planet.color);
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

        let (x, y) = camera().world_to_screen(SHIP.x, SHIP.y);
        circ(x, y, 2, 4);

        let mut prev_step = [SHIP.x, SHIP.y];
        for (idx, step) in simulate_trajectory(time(), &SHIP, &PLANETS)
            .iter()
            .enumerate()
        {
            if idx % 100 == 0 {
                let (x1, y1) = camera().world_to_screen(prev_step[0], prev_step[1]);
                let (x2, y2) = camera().world_to_screen(step.x, step.y);

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
