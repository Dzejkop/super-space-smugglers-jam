mod alloc;
mod tic80;

use std::sync::Mutex;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use tic80::*;

static mut T: i32 = 0;
static mut FIRE_SPRITE: bool = true;

lazy_static::lazy_static! {
    static ref RNG: Mutex<SmallRng> = Mutex::new(SmallRng::seed_from_u64(42));
}

struct Ship {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

static mut SHIP_STATE: Ship = Ship {
    x: 0.0,
    y: 0.0,
    vx: 0.0,
    vy: 0.0,
};

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    sprite: i32,
    life: i32,
}

impl Particle {
    const fn null() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            sprite: -1,
            life: 0,
        }
    }
}

const NUM_PARTICLES: usize = 255;
static mut THRUSTER_PARTICLES: [Particle; NUM_PARTICLES] = [Particle::null(); NUM_PARTICLES];

const INITIAL_PARTICLE_VELOCITY: f32 = 6.0;
const PARTICLE_LIFE: i32 = 60;
const PARTICLE_SLOWDOWN: f32 = 0.8;

const PARTICLE_SPREAD: f32 = 3.0;
const PARTICLE_VARIANCE: f32 = 2.5;

mod sprites {
    pub const SHIP: i32 = 292;
    pub const LEFT_THRUSTER_1: i32 = 290;
    pub const LEFT_THRUSTER_2: i32 = 258;
    pub const RIGHT_THRUSTER_1: i32 = 294;
    pub const RIGHT_THRUSTER_2: i32 = 296;
    pub const FIRE_1: i32 = 324;
    pub const FIRE_2: i32 = 326;
    pub const TOP_THRUSTER_1: i32 = 260;
    pub const TOP_THRUSTER_2: i32 = 262;

    pub const THRUSTER_PARTICLES: &[i32] = &[264, 265, 266, 267, 268, 269];
}

mod btns {
    pub const UP: i32 = 0;
    pub const DOWN: i32 = 1;
    pub const LEFT: i32 = 2;
    pub const RIGHT: i32 = 3;
}

#[export_name = "TIC"]
pub fn tic() {
    let mut rng = RNG.lock().unwrap();

    cls(0);

    unsafe {
        let x = SHIP_STATE.x as i32 - 12;
        let y = SHIP_STATE.y as i32 - 16;

        spr(
            sprites::SHIP,
            x,
            y,
            SpriteOptions {
                // scale: 2,
                transparent: &[0],
                w: 2,
                h: 2,
                ..Default::default()
            },
        );

        if btn(btns::DOWN) {
            SHIP_STATE.vy += 0.05;

            sfx(
                2,
                SfxOptions {
                    note: 45,
                    duration: 4,
                    channel: 3,
                    ..default()
                },
            );

            spr(
                if FIRE_SPRITE {
                    sprites::TOP_THRUSTER_1
                } else {
                    sprites::TOP_THRUSTER_2
                },
                x,
                y - 16,
                SpriteOptions {
                    // scale: 2,
                    transparent: &[0],
                    w: 2,
                    h: 2,
                    ..Default::default()
                },
            );

            spawn_particle(
                SHIP_STATE.x - 8.0,
                SHIP_STATE.y - 20.0,
                (rng.gen::<f32>() - 0.5) * PARTICLE_SPREAD,
                -INITIAL_PARTICLE_VELOCITY + (rng.gen::<f32>() - 0.5) * PARTICLE_VARIANCE,
                &mut rng,
            );
        }

        if btn(btns::RIGHT) {
            SHIP_STATE.vx += 0.05;

            sfx(
                2,
                SfxOptions {
                    note: 45,
                    duration: 4,
                    channel: 3,
                    ..default()
                },
            );

            spr(
                if FIRE_SPRITE {
                    sprites::LEFT_THRUSTER_1
                } else {
                    sprites::LEFT_THRUSTER_2
                },
                x - 16,
                y,
                SpriteOptions {
                    // scale: 2,
                    transparent: &[0],
                    w: 2,
                    h: 2,
                    ..Default::default()
                },
            );

            spawn_particle(
                SHIP_STATE.x - 16.0,
                SHIP_STATE.y - 10.0,
                -INITIAL_PARTICLE_VELOCITY + (rng.gen::<f32>() - 0.5) * PARTICLE_VARIANCE,
                (rng.gen::<f32>() - 0.5) * PARTICLE_SPREAD,
                &mut rng,
            );
        }

        if btn(btns::LEFT) {
            SHIP_STATE.vx -= 0.05;

            sfx(
                2,
                SfxOptions {
                    note: 45,
                    duration: 4,
                    channel: 3,
                    ..default()
                },
            );

            spr(
                if FIRE_SPRITE {
                    sprites::RIGHT_THRUSTER_1
                } else {
                    sprites::RIGHT_THRUSTER_2
                },
                x + 16,
                y,
                SpriteOptions {
                    // scale: 2,
                    transparent: &[0],
                    w: 2,
                    h: 2,
                    ..Default::default()
                },
            );

            spawn_particle(
                SHIP_STATE.x,
                SHIP_STATE.y - 10.0,
                INITIAL_PARTICLE_VELOCITY + (rng.gen::<f32>() - 0.5) * PARTICLE_VARIANCE,
                (rng.gen::<f32>() - 0.5) * PARTICLE_SPREAD,
                &mut rng,
            );
        }

        if btn(btns::UP) {
            SHIP_STATE.vy -= 0.1;

            sfx(
                1,
                SfxOptions {
                    note: 12,
                    duration: 4,
                    channel: 2,
                    ..default()
                },
            );

            spr(
                if FIRE_SPRITE {
                    sprites::FIRE_1
                } else {
                    sprites::FIRE_2
                },
                x,
                y + 16,
                SpriteOptions {
                    // scale: 2,
                    transparent: &[0],
                    w: 2,
                    h: 2,
                    ..Default::default()
                },
            );

            spawn_particle(
                SHIP_STATE.x - 8.0,
                SHIP_STATE.y + 2.0,
                (rng.gen::<f32>() - 0.5) * PARTICLE_SPREAD,
                INITIAL_PARTICLE_VELOCITY + (rng.gen::<f32>() - 0.5) * PARTICLE_VARIANCE,
                &mut rng,
            );
        }

        for particle in &mut THRUSTER_PARTICLES {
            if particle.sprite != -1 {
                particle.life -= 1;

                let index = if particle.life < 10 {
                    0
                } else if particle.life < 20 {
                    1
                } else if particle.life < 30 {
                    2
                } else if particle.life < 45 {
                    3
                } else if particle.life < 55 {
                    4
                } else {
                    5
                };

                particle.sprite = sprites::THRUSTER_PARTICLES[index];

                particle.x += particle.vx;
                particle.y += particle.vy;

                particle.vx *= PARTICLE_SLOWDOWN;
                particle.vy *= PARTICLE_SLOWDOWN;

                spr(
                    particle.sprite,
                    particle.x as i32 - 4,
                    particle.y as i32 - 4,
                    SpriteOptions {
                        transparent: &[0],
                        scale: 2,
                        ..default()
                    },
                );

                if particle.life <= 0 {
                    particle.sprite = -1;
                }
            }
        }

        SHIP_STATE.vy += 0.01;
    }

    print!("Hello, World!", 0, 0, PrintOptions::default());

    unsafe {
        T += 1;
        if T > 4 {
            T = 0;
            FIRE_SPRITE = !FIRE_SPRITE;
        }

        SHIP_STATE.x += SHIP_STATE.vx;
        SHIP_STATE.y += SHIP_STATE.vy;

        if SHIP_STATE.x as i32 > WIDTH {
            SHIP_STATE.vx = -SHIP_STATE.vx;
        } else if SHIP_STATE.x < 0.0 {
            SHIP_STATE.vx = -SHIP_STATE.vx;
        }

        if SHIP_STATE.y as i32 > HEIGHT {
            SHIP_STATE.vy = -SHIP_STATE.vy;
        } else if SHIP_STATE.y < 0.0 {
            SHIP_STATE.vy = -SHIP_STATE.vy;
        }
    }
}

fn spawn_particle(x: f32, y: f32, vx: f32, vy: f32, rng: &mut SmallRng) {
    unsafe {
        for particle in &mut THRUSTER_PARTICLES {
            if particle.sprite == -1 {
                particle.x = x;
                particle.y = y;
                particle.vx = vx;
                particle.vy = vy;
                particle.sprite = sprites::THRUSTER_PARTICLES[3];
                particle.life = PARTICLE_LIFE;

                break;
            }
        }
    }
}

fn default<T>() -> T
where
    T: Default,
{
    T::default()
}
