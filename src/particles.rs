use crate::prelude::*;

pub fn tic(rng: &mut dyn RngCore) {
    let particles = unsafe { &mut PARTICLES };

    for particle in particles {
        if particle.life == 0 {
            continue;
        }

        particle.life -= 1;

        let sprite = lerp(
            particle.max_sprite_idx as f32,
            particle.min_sprite_idx as f32,
            particle.life as f32 / particle.max_life as f32,
        ) as u32;

        Img::sprite_idx(sprite).at(particle.pos).draw();

        particle.pos += particle.vel;
        particle.vel *= vec2(rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0));
        particle.vel +=
            vec2(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1));
    }
}

pub fn spawn(
    pos: Vec2,
    vel: Vec2,
    min_sprite_idx: u32,
    max_sprite_idx: u32,
    life: u32,
) {
    let particles = unsafe { &mut PARTICLES };

    for particle in particles {
        if particle.life == 0 {
            *particle = Particle {
                pos,
                vel,
                max_sprite_idx,
                min_sprite_idx,
                life,
                max_life: life,
            };

            break;
        }
    }
}

pub fn is_any_visible() -> bool {
    let particles = unsafe { &PARTICLES };

    for particle in particles {
        if particle.life == 0 {
            continue;
        }

        if particle.pos.x >= -4.0
            && particle.pos.y >= -4.0
            && particle.pos.x <= (WIDTH as f32) + 4.0
            && particle.pos.y <= (HEIGHT as f32) + 4.0
        {
            return true;
        }
    }

    false
}

static mut PARTICLES: [Particle; 128] = [Particle::null(); 128];

#[derive(Clone, Copy)]
struct Particle {
    pos: Vec2,
    vel: Vec2,
    min_sprite_idx: u32,
    max_sprite_idx: u32,
    life: u32,
    max_life: u32,
}

impl Particle {
    const fn null() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
            vel: vec2(0.0, 0.0),
            min_sprite_idx: 0,
            max_sprite_idx: 0,
            life: 0,
            max_life: 0,
        }
    }
}
