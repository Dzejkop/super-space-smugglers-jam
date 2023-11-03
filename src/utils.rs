use crate::tic80::*;
use glam::*;

pub struct Img {
    uv_min: Vec2,
    uv_max: Vec2,
    at: Vec2,
    rot: f32,
    scale: f32,
}

impl Img {
    pub fn sprite(id: UVec2, size: UVec2) -> Self {
        let id = 16 + id;
        let id_min = id;
        let id_max = id + size;

        Self {
            uv_min: id_min.as_vec2() * 8.0,
            uv_max: id_max.as_vec2() * 8.0,
            at: vec2(0.0, 0.0),
            rot: 0.0,
            scale: 1.0,
        }
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

    pub fn draw(self) {
        let Self {
            uv_min,
            uv_max,
            at,
            rot,
            scale,
        } = self;

        let size = (uv_max - uv_min) * scale;
        let transform = |vertex| rotate(vertex, at + size * 0.5, rot);

        let v0 = at;
        let v1 = vec2(at.x + size.x, at.y);
        let v2 = vec2(at.x, at.y + size.y);
        let v3 = at + size;

        let uv0 = uv_min;
        let uv1 = vec2(uv_max.x, uv_min.y);
        let uv2 = vec2(uv_min.x, uv_max.y);
        let uv3 = uv_max;

        let vertices = [transform(v0), transform(v1), transform(v2), transform(v3)];
        let uvs = [uv0, uv1, uv2, uv3];

        let opts = TTriOptions {
            texture_src: TextureSource::Tiles,
            transparent: &[],
            z1: 0.0,
            z2: 0.0,
            z3: 0.0,
            depth: false,
        };

        let tris = [[0, 1, 2], [2, 1, 3]];

        for [id0, id1, id2] in tris {
            ttri(
                vertices[id0].x,
                vertices[id0].y,
                vertices[id1].x,
                vertices[id1].y,
                vertices[id2].x,
                vertices[id2].y,
                uvs[id0].x,
                uvs[id0].y,
                uvs[id1].x,
                uvs[id1].y,
                uvs[id2].x,
                uvs[id2].y,
                opts,
            );
        }
    }
}

pub fn rotate(point: Vec2, around: Vec2, angle: f32) -> Vec2 {
    (point - around).rotate(vec2(angle.cos(), angle.sin())) + around
}
