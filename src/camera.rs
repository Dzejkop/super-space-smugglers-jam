use crate::prelude::*;

pub const MAX_ZOOM: f32 = 1.5;
pub const MIN_ZOOM: f32 = 1e-2;

static mut CAMERA: Camera = Camera {
    pos: vec2(0.0, 0.0),
    zoom: 0.2,
};

pub unsafe fn get() -> &'static Camera {
    &CAMERA
}

pub unsafe fn get_mut() -> &'static mut Camera {
    &mut CAMERA
}

pub fn tic() {
    const SPEED: f32 = 2.0;

    let camera = unsafe { get_mut() };

    let m = mouse();

    if key(keys::A) {
        camera.pos.x += SPEED / camera.zoom;
    }

    if key(keys::D) {
        camera.pos.x -= SPEED / camera.zoom;
    }

    if key(keys::W) {
        camera.pos.y += SPEED / camera.zoom;
    }

    if key(keys::S) {
        camera.pos.y -= SPEED / camera.zoom;
    }

    if m.scroll_y != 0 {
        let world_pos = camera.screen_to_world(m.x as i32, m.y as i32);

        let screen_pos_before =
            camera.world_to_screen(world_pos.0, world_pos.1);

        if m.scroll_y > 0 {
            camera.zoom *= 1.2;
        } else {
            camera.zoom /= 1.2;
        }

        camera.zoom = camera.zoom.clamp(MIN_ZOOM, MAX_ZOOM);

        let screen_pos_after = camera.world_to_screen(world_pos.0, world_pos.1);

        camera.pos.x -=
            (screen_pos_after.0 - screen_pos_before.0) / camera.zoom;

        camera.pos.y -=
            (screen_pos_after.1 - screen_pos_before.1) / camera.zoom;
    }
}

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
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
