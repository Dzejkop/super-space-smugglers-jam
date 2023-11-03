use crate::tic80::*;
use glam::*;

pub struct Sprite {
    uv_min: Vec2,
    uv_max: Vec2,
    at: Vec2,
    size: Vec2,
}

impl Sprite {
    pub fn slice(id_min: UVec2, id_max: UVec2) -> Self {
        Self {
            uv_min: id_min.as_vec2() * 8.0,
            uv_max: (id_max.as_vec2() + 1.0) * 8.0,
            at: vec2(0.0, 0.0),
            size: ((id_max - id_min).as_vec2() + 1.0) * 8.0,
        }
    }

    pub fn at(mut self, at: Vec2) -> Self {
        self.at = at;
        self
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn render(self) {
        let opts = TTriOptions {
            texture_src: TextureSource::Tiles,
            transparent: &[],
            z1: 0.0,
            z2: 0.0,
            z3: 0.0,
            depth: false,
        };

        ttri(
            self.at.x + 0.0,
            self.at.y + 0.0,
            self.at.x + self.size.x,
            self.at.y + 0.0,
            self.at.x + 0.0,
            self.at.y + self.size.y,
            self.uv_min.x,
            self.uv_min.y,
            self.uv_max.x,
            self.uv_min.y,
            self.uv_min.x,
            self.uv_max.y,
            opts,
        );

        ttri(
            self.at.x + 0.0,
            self.at.y + self.size.y,
            self.at.x + self.size.x,
            self.at.y + 0.0,
            self.at.x + self.size.x,
            self.at.y + self.size.y,
            self.uv_min.x,
            self.uv_max.y,
            self.uv_max.x,
            self.uv_min.y,
            self.uv_max.x,
            self.uv_max.y,
            opts,
        );
    }
}
