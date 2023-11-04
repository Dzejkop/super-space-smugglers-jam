use std::ops;

use crate::prelude::*;

pub mod arrow;

pub mod btns {
    pub const UP: i32 = 0;
    pub const DOWN: i32 = 1;
    pub const LEFT: i32 = 2;
    pub const RIGHT: i32 = 3;
}

pub mod keys {
    pub const A: i32 = 1;
    pub const B: i32 = 2;
    pub const C: i32 = 3;
    pub const D: i32 = 4;
    pub const E: i32 = 5;
    pub const F: i32 = 6;
    pub const G: i32 = 7;
    pub const H: i32 = 8;
    pub const I: i32 = 9;
    pub const J: i32 = 10;
    pub const K: i32 = 11;
    pub const L: i32 = 12;
    pub const M: i32 = 13;
    pub const N: i32 = 14;
    pub const O: i32 = 15;
    pub const P: i32 = 16;
    pub const Q: i32 = 17;
    pub const R: i32 = 18;
    pub const S: i32 = 19;
    pub const T: i32 = 20;
    pub const U: i32 = 21;
    pub const V: i32 = 22;
    pub const W: i32 = 23;
    pub const X: i32 = 24;
    pub const Y: i32 = 25;
    pub const Z: i32 = 26;

    pub const DIGIT_0: i32 = 27;
    pub const DIGIT_1: i32 = 28;
    pub const DIGIT_2: i32 = 29;
    pub const DIGIT_3: i32 = 30;
    pub const DIGIT_4: i32 = 31;
    pub const DIGIT_5: i32 = 32;
    pub const DIGIT_6: i32 = 33;
    pub const DIGIT_7: i32 = 34;
    pub const DIGIT_8: i32 = 35;
    pub const DIGIT_9: i32 = 36;

    pub const SPACE: i32 = 48;
}

pub mod sprites {
    pub mod buttons {
        pub mod active {
            pub const STOP: i32 = 64;
            pub const NORMAL: i32 = 66;
            pub const FAST: i32 = 68;
        }

        pub mod highlighted {
            pub const STOP: i32 = 32;
            pub const NORMAL: i32 = 34;
            pub const FAST: i32 = 36;
        }

        pub mod inactive {
            pub const STOP: i32 = 0;
            pub const NORMAL: i32 = 2;
            pub const FAST: i32 = 4;
        }
    }
}

pub struct Img {
    uv_min: Vec2,
    uv_max: Vec2,
    at: Vec2,
    rot: f32,
    scale: f32,
}

impl Img {
    pub fn sprite(id: UVec2, size: UVec2) -> Self {
        Self {
            uv_min: id.as_vec2() * 8.0,
            uv_max: (id + size).as_vec2() * 8.0,
            at: vec2(0.0, 0.0),
            rot: 0.0,
            scale: 1.0,
        }
    }

    pub fn sprite_idx(idx: u32) -> Self {
        Self::sprite(uvec2(idx % 16, idx / 16), uvec2(1, 1))
    }

    pub fn sprite_idx_with_size(idx: u32, size: UVec2) -> Self {
        Self::sprite(uvec2(idx % 16, idx / 16), size)
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
        let transform =
            |vertex| rotate(vertex, at + size * 0.5, rot) - size * 0.5;

        let v0 = at;
        let v1 = vec2(at.x + size.x, at.y);
        let v2 = vec2(at.x, at.y + size.y);
        let v3 = at + size;

        let uv0 = uv_min;
        let uv1 = vec2(uv_max.x, uv_min.y);
        let uv2 = vec2(uv_min.x, uv_max.y);
        let uv3 = uv_max;

        let vertices =
            [transform(v0), transform(v1), transform(v2), transform(v3)];
        let uvs = [uv0, uv1, uv2, uv3];

        let opts = TTriOptions {
            texture_src: TextureSource::Tiles,
            transparent: &[0],
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

pub fn lerp<T>(a: T, b: T, t: f32) -> T
where
    T: ops::Add<Output = T>,
    T: ops::Sub<Output = T>,
    T: ops::Mul<f32, Output = T>,
    T: Copy,
{
    a + (b - a) * t.clamp(0.0, 1.0)
}
