use crate::prelude::*;

pub struct SelectionIndicator {
    at: Vec2,
    size: Vec2,
}

impl SelectionIndicator {
    pub fn new(at: Vec2) -> Self {
        Self {
            at,
            size: Default::default(),
        }
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn draw(self) {
        let Self { at, size } = self;
        let hsize = size / 2.0;

        let v0 = at - hsize;
        let v1 = v0 + vec2(0.0, hsize.y * 0.33);
        let v2 = v0 + vec2(hsize.x * 0.33, 0.0);

        for rot in [0.0, PI / 2.0, PI, 1.5 * PI] {
            let v0 = rotate(v0, at, rot);
            let v1 = rotate(v1, at, rot);
            let v2 = rotate(v2, at, rot);

            line(v0.x, v0.y, v1.x, v1.y, 5);
            line(v0.x, v0.y, v2.x, v2.y, 5);
        }
    }
}
