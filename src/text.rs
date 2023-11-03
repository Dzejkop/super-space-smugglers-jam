use crate::print;
use crate::tic80::*;
use glam::*;

pub struct Text {
    text: String,
    at: Vec2,
    color: i32,
    alignment: TextAlignment,
}

impl Text {
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
            at: Default::default(),
            color: 12,
            alignment: Default::default(),
        }
    }

    pub fn at(mut self, at: Vec2) -> Self {
        self.at = at;
        self
    }

    pub fn color(mut self, color: i32) -> Self {
        self.color = color;
        self
    }

    pub fn centered(mut self) -> Self {
        self.alignment = TextAlignment::Center;
        self
    }

    pub fn draw(self) {
        let Self {
            text,
            at,
            color,
            alignment,
        } = self;

        let at = match alignment {
            TextAlignment::Left => at,

            TextAlignment::Center => {
                let width = print_alloc(&text, 1024, 1024, Default::default());

                vec2((at.x - width as f32) / 2.0, at.y)
            }
        };

        let opts = PrintOptions {
            color,
            ..Default::default()
        };

        print!(text, at.x as i32, at.y as i32, opts);
    }
}

enum TextAlignment {
    Left,
    Center,
}

impl Default for TextAlignment {
    fn default() -> Self {
        TextAlignment::Center
    }
}
