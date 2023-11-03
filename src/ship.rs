use crate::{tic80::*, utils::*};
use glam::*;

pub struct ShipSprite {
    id: ShipSpriteId,
    at: Vec2,
    rot: f32,
    engine: bool,
}

impl ShipSprite {
    fn new(id: ShipSpriteId) -> Self {
        Self {
            id,
            at: Default::default(),
            rot: Default::default(),
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

    pub fn engine(mut self, engine: bool) -> Self {
        self.engine = engine;
        self
    }

    pub fn draw(self) -> Vec2 {
        let Self {
            id,
            at,
            rot,
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

        Img::sprite(sprite, uvec2(2, 2)).at(at).rot(rot).draw();

        let engine_at = rotate(at + vec2(0.0, 16.0), at, rot);

        if engine {
            Img::sprite(uvec2(16, 18), uvec2(2, 2))
                .at(engine_at)
                .rot(rot)
                .draw();
        }

        engine_at + vec2(4.0, 4.0)
    }
}

enum ShipSpriteId {
    Player,
    Police,
}
