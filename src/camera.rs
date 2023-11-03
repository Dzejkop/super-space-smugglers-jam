use crate::tic80::{WIDTH, HEIGHT};

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

static mut CAMERA: Camera = Camera {
    x: 0.0,
    y: 0.0,
    zoom: 1e-2,
};

pub fn camera() -> &'static Camera {
    unsafe { &CAMERA }
}

pub fn camera_mut() -> &'static mut Camera {
    unsafe { &mut CAMERA }
}

impl Camera {
    pub fn world_to_screen(&self, x: f32, y: f32) -> (i32, i32) {
        let cx = WIDTH / 2;
        let cy = HEIGHT / 2;

        (
            ((self.x + x) * self.zoom + cx as f32) as i32,
            ((self.y + y) * self.zoom + cy as f32) as i32,
        )
    }
}
