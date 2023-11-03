use crate::utils::*;
use glam::*;
use rand::{Rng, RngCore};

pub fn tic(rng: &mut dyn RngCore) {
    let particles = unsafe { &mut PARTICLES };

    for particle in particles {
        if particle.life == 0 {
            continue;
        }

        particle.life -= 1;

        let sprite = lerp(
            particle.max_sprite as f32,
            particle.min_sprite as f32,
            particle.life as f32 / particle.max_life as f32,
        ) as u32;

        Img::sprite(sprite).at(particle.pos).draw();

        particle.pos += particle.vel;
        particle.vel *= vec2(rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0));
        particle.vel += vec2(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1));
    }
}

pub fn spawn(pos: Vec2, vel: Vec2, min_sprite: u32, max_sprite: u32, life: u32) {
    let particles = unsafe { &mut PARTICLES };

    for particle in particles {
        if particle.life == 0 {
            *particle = Particle {
                pos,
                vel,
                max_sprite,
                min_sprite,
                life,
                max_life: life,
            };

            break;
        }
    }
}

static mut PARTICLES: [Particle; 256] = [Particle::null(); 256];

#[derive(Clone, Copy)]
struct Particle {
    pos: Vec2,
    vel: Vec2,
    min_sprite: u32,
    max_sprite: u32,
    life: u32,
    max_life: u32,
}

impl Particle {
    const fn null() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
            vel: vec2(0.0, 0.0),
            min_sprite: 0,
            max_sprite: 0,
            life: 0,
            max_life: 0,
        }
    }
}
