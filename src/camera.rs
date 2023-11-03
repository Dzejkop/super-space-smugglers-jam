use crate::tic80::{HEIGHT, WIDTH};

pub const MAX_ZOOM: f32 = 1.5;
pub const MIN_ZOOM: f32 = 1e-1;
pub const ZOOM_LERP: f32 = 0.1;

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

static mut CAMERA: Camera = Camera {
    x: 0.0,
    y: 0.0,
    zoom: 1.0,
};

pub fn camera() -> &'static Camera {
    unsafe { &CAMERA }
}

pub fn camera_mut() -> &'static mut Camera {
    unsafe { &mut CAMERA }
}

impl Camera {
    // Maps self.zoom which is between 0 and 1 to MIN_ZOOM and MAX_ZOOM
    pub fn remap_zoom(&self) -> f32 {
        self.zoom * (MAX_ZOOM - MIN_ZOOM) + MIN_ZOOM
    }

    pub fn world_to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        let cx = WIDTH / 2;
        let cy = HEIGHT / 2;

        let zoom = self.remap_zoom();

        (
            (self.x + x) * zoom + cx as f32,
            (self.y + y) * zoom + cy as f32,
        )
    }

    pub fn world_to_screen_integer(&self, x: f32, y: f32) -> (i32, i32) {
        let (x, y) = self.world_to_screen(x, y);

        (x as i32, y as i32)
    }

    pub fn screen_to_world(&self, x: i32, y: i32) -> (f32, f32) {
        let cx = WIDTH / 2;
        let cy = HEIGHT / 2;

        let zoom = self.remap_zoom();

        (
            (x as f32 - cx as f32) / zoom - self.x,
            (y as f32 - cy as f32) / zoom - self.y,
        )
    }
}
