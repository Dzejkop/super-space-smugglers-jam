use crate::prelude::*;

static mut CAMERA: Camera = Camera {
    pos: vec2(0.0, 0.0),
    zoom: 0.2,
    target_zoom: 0.2,
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
        if m.scroll_y > 0 {
            camera.target_zoom *= 1.2;
        } else {
            camera.target_zoom /= 1.2;
        }

        camera.target_zoom = camera.target_zoom.clamp(0.01, 1.5);
    }

    // ---

    let zoom_diff = camera.target_zoom - camera.zoom;

    if zoom_diff.abs() > 0.001 {
        let world_pos = camera.screen_to_world(vec2(m.x as f32, m.y as f32));
        let screen_pos_before = camera.world_to_screen(world_pos);

        camera.zoom += zoom_diff * 0.25;

        let screen_pos_after = camera.world_to_screen(world_pos);

        camera.pos -= (screen_pos_after - screen_pos_before) / camera.zoom;
    }
}

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
    pub target_zoom: f32,
}

impl Camera {
    fn center() -> Vec2 {
        vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0)
    }

    pub fn world_to_screen(&self, pos: Vec2) -> Vec2 {
        Self::center() + (self.pos + pos) * self.zoom
    }

    pub fn screen_to_world(&self, pos: Vec2) -> Vec2 {
        (pos - Self::center()) / self.zoom - self.pos
    }
}
