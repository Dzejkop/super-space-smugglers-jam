use crate::prelude::*;

const ARROW_HEAD_SIZE: f32 = 10.0;
const ARROW_HEAD_ANGLE: f32 = PI / 6.0;

pub struct Arrow {
    start: Vec2,
    end: Vec2,
    color: u8,
    margin: f32,
}

impl Arrow {
    pub fn new(start: Vec2, end: Vec2, color: u8) -> Self {
        Self {
            start,
            end,
            color,
            margin: 0.0,
        }
    }

    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    pub fn draw(self) {
        let to_end = self.end - self.start;

        let start = self.start + to_end.normalize() * self.margin;
        let end = self.end - to_end.normalize() * self.margin;

        line(start.x, start.y, end.x, end.y, self.color);

        let ear = start - end;
        let ear = ear.normalize() * ARROW_HEAD_SIZE;

        let ear = end + ear;

        let ear_left = rotate(ear, end, ARROW_HEAD_ANGLE);
        let ear_right = rotate(ear, end, -ARROW_HEAD_ANGLE);

        line(end.x, end.y, ear_left.x, ear_left.y, self.color);
        line(end.x, end.y, ear_right.x, ear_right.y, self.color);
    }
}
