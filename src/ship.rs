use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Ship {
    pub pos: Vec2,
    pub vel: Vec2,
}

#[derive(Clone, Copy)]
pub struct ShipSprite {
    ty: ShipSpriteTy,
    at: Vec2,
    rot: f32,
    scale: f32,
    engine: bool,
}

impl ShipSprite {
    fn new(ty: ShipSpriteTy) -> Self {
        Self {
            ty,
            at: Default::default(),
            rot: Default::default(),
            scale: 1.0,
            engine: Default::default(),
        }
    }

    pub fn player() -> Self {
        Self::new(ShipSpriteTy::Player)
    }

    pub fn police(in_pursuit: bool) -> Self {
        Self::new(ShipSpriteTy::Police { in_pursuit })
    }

    pub fn at(mut self, at: Vec2) -> Self {
        self.at = at;
        self
    }

    pub fn rot(mut self, rot: f32) -> Self {
        self.rot = rot;
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn engine(mut self, engine: bool) -> Self {
        self.engine = engine;
        self
    }

    pub fn draw(self, game: Option<&Game>) -> Vec2 {
        let Self {
            ty: id,
            at,
            rot,
            scale,
            engine,
        } = self;

        let sprite = match id {
            ShipSpriteTy::Player => uvec2(16, 16),

            ShipSpriteTy::Police { in_pursuit } => {
                if in_pursuit {
                    if police_alternate_sprite() {
                        uvec2(18, 16)
                    } else {
                        uvec2(20, 16)
                    }
                } else {
                    uvec2(16, 24)
                }
            }
        };

        Img::sprite(sprite, uvec2(2, 2))
            .at(at)
            .rot(rot)
            .scale(scale)
            .draw();

        let engine_at = rotate(at + vec2(0.0, 16.0) * scale, at, rot);

        if engine {
            let time = game.map(|game| game.time).unwrap_or_else(time);

            let sprite_idx = if (time / 100.0) as i32 % 2 == 0 {
                288
            } else {
                290
            };

            Img::sprite_idx_with_size(sprite_idx, uvec2(2, 2))
                .at(engine_at)
                .rot(rot)
                .scale(scale)
                .draw();
        }

        engine_at
    }
}

pub fn police_alternate_sprite() -> bool {
    time() % 600.0 < 300.0
}

#[derive(Clone, Copy)]
enum ShipSpriteTy {
    Player,
    Police { in_pursuit: bool },
}
