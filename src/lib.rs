mod alloc;
mod orbits;
mod particles;
mod tic80;
mod utils;

use self::tic80::sys::print;
use self::tic80::*;
use self::utils::*;
use crate::orbits::simulate_trajectory;
use glam::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;

const SHIP_MASS: f32 = 0.0001;

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
        radius: 7.0,
        mass: 100.0,
        color: 6,
    },
    Planet {
        x: 0.0,
        y: 0.0,
        orbit_radius: 30.0,
        orbit_speed: 0.0002,
        radius: 5.0,
        mass: 10.0,
        color: 2,
    },
    Planet {
        x: 0.0,
        y: 0.0,
        orbit_radius: 40.0,
        orbit_speed: 0.00001,
        radius: 2.0,
        mass: 1.0,
        color: 3,
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
    let cx = WIDTH / 2;
    let cy = HEIGHT / 2;

    circ(cx, cy, 10, 1);

    for planet in unsafe { &mut PLANETS.iter_mut() } {
        planet.x = cx as f32 + f32::sin(time() * planet.orbit_speed) * planet.orbit_radius;
        planet.y = cy as f32 + f32::cos(time() * planet.orbit_speed) * planet.orbit_radius;

        // Draw orbit
        circb(cx, cy, planet.orbit_radius as i32, planet.color);

        // Draw planet
        circ(
            planet.x as i32,
            planet.y as i32,
            planet.radius as i32,
            planet.color,
        );
    }

    let m = mouse();
    if m.left && !unsafe { MOUSE_LEFT_PREV } {
        unsafe {
            SHIP.x = m.x as f32;
            SHIP.y = m.y as f32;

            SHIP.vx = 0.0;
            SHIP.vy = 0.1;
        }
    }

    // Draw the ship
    unsafe {
        for planet in &PLANETS {
            let dx = planet.x - SHIP.x;
            let dy = planet.y - SHIP.y;
            let d2 = dx * dx + dy * dy;

            line(SHIP.x, SHIP.y, SHIP.x + dx, SHIP.y + dy, 7);

            let f = planet.mass * SHIP_MASS / d2;

            SHIP.vx += f * dx;
            SHIP.vy += f * dy;
        }

        SHIP.x += SHIP.vx;
        SHIP.y += SHIP.vy;

        const M: f32 = 100.0;
        line(
            SHIP.x,
            SHIP.y,
            SHIP.x + SHIP.vx * M,
            SHIP.y + SHIP.vy * M,
            6,
        );

        circ(SHIP.x as i32, SHIP.y as i32, 2, 4);

        for (idx, step) in simulate_trajectory(time(), &SHIP, &PLANETS)
            .iter()
            .enumerate()
        {
            if idx % 25 == 0 {
                circ(step.x as i32, step.y as i32, 1, 7);
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
