use crate::tic80::{HEIGHT, WIDTH};
use glam::*;

pub const MAX_ZOOM: f32 = 1.5;
pub const MIN_ZOOM: f32 = 1e-2;

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
}

static mut CAMERA: Camera = Camera {
    pos: vec2(0.0, 0.0),
    zoom: 1.0,
};

pub fn camera() -> &'static Camera {
    unsafe { &CAMERA }
}

pub fn camera_mut() -> &'static mut Camera {
    unsafe { &mut CAMERA }
}

impl Camera {
    pub fn world_to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        let cx = WIDTH / 2;
        let cy = HEIGHT / 2;

        (
            (self.pos.x + x) * self.zoom + cx as f32,
            (self.pos.y + y) * self.zoom + cy as f32,
        )
    }

    pub fn world_to_screen_integer(&self, x: f32, y: f32) -> (i32, i32) {
        let (x, y) = self.world_to_screen(x, y);

        (x as i32, y as i32)
    }

    pub fn screen_to_world(&self, x: i32, y: i32) -> (f32, f32) {
        let cx = WIDTH / 2;
        let cy = HEIGHT / 2;

        (
            (x as f32 - cx as f32) / self.zoom - self.pos.x,
            (y as f32 - cy as f32) / self.zoom - self.pos.y,
        )
    }
}
