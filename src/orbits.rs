use crate::tic80::{HEIGHT, WIDTH};
use crate::{Planet, Ship, SHIP_MASS};

const TIME_STEP: f32 = 1000.0 / 60.0;
const NUM_STEPS: usize = 10_000;

#[derive(Clone, Copy)]
pub struct TrajectoryStep {
    pub t: f32,
    pub x: f32,
    pub y: f32,
}

static mut TRAJECTORY: [TrajectoryStep; NUM_STEPS] = [TrajectoryStep {
    t: 0.0,
    x: 0.0,
    y: 0.0,
}; NUM_STEPS];

pub fn simulate_trajectory(t: f32, ship: &Ship, planets: &[Planet]) -> &'static [TrajectoryStep] {
    let mut planets = planets.to_vec();
    let mut ship = ship.clone();

    let cx = WIDTH / 2;
    let cy = HEIGHT / 2;

    for n in 0..NUM_STEPS {
        let time = t + TIME_STEP * n as f32;

        for planet in &mut planets {
            planet.x = cx as f32 + f32::sin(time * planet.orbit_speed) * planet.orbit_radius;
            planet.y = cy as f32 + f32::cos(time * planet.orbit_speed) * planet.orbit_radius;
        }

        for planet in &planets {
            let dx = planet.x - ship.x;
            let dy = planet.y - ship.y;
            let d2 = dx * dx + dy * dy;

            let f = planet.mass * SHIP_MASS / d2;

            ship.vx += f * dx;
            ship.vy += f * dy;
        }

        ship.x += ship.vx;
        ship.y += ship.vy;

        unsafe {
            TRAJECTORY[n].t = time;
            TRAJECTORY[n].x = ship.x;
            TRAJECTORY[n].y = ship.y;
        }
    }

    unsafe { &TRAJECTORY }
}
