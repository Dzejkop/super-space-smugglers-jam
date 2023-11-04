use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Ship {
    pub pos: Vec2,
    pub vel: Vec2,

    // Index of planet
    pub in_orbit: Option<usize>,
}

pub struct ShipSprite {
    id: ShipSpriteId,
    at: Vec2,
    rot: f32,
    scale: f32,
    engine: bool,
}

impl ShipSprite {
    fn new(id: ShipSpriteId) -> Self {
        Self {
            id,
            at: Default::default(),
            rot: Default::default(),
            scale: 1.0,
            engine: Default::default(),
        }
    }

    pub fn player() -> Self {
        Self::new(ShipSpriteId::Player)
    }

    pub fn police() -> Self {
        Self::new(ShipSpriteId::Police)
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

    pub fn draw(self) -> Vec2 {
        let Self {
            id,
            at,
            rot,
            scale,
            engine,
        } = self;

        let sprite = match id {
            ShipSpriteId::Player => uvec2(16, 16),
            ShipSpriteId::Police => {
                if time() % 600.0 < 300.0 {
                    uvec2(18, 16)
                } else {
                    uvec2(20, 16)
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
            Img::sprite(uvec2(16, 18), uvec2(2, 2))
                .at(engine_at)
                .rot(rot)
                .scale(scale)
                .draw();
        }

        engine_at
    }
}

enum ShipSpriteId {
    Player,
    Police,
}
