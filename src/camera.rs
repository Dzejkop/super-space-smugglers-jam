use crate::prelude::*;

static mut CAMERA: Camera = Camera {
    pos: vec2(0.0, 0.0),
    scale: 0.033,
    target_scale: 0.033,
    anim_target: None,
    anim_origin: None,
    anim_t: 0.0,
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

    if let (Some(origin), Some(target)) =
        (camera.anim_origin, camera.anim_target)
    {
        fn ease(t: f32) -> f32 {
            if t < 0.5 {
                4.0 * t * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powf(3.0) / 2.0
            }
        }

        camera.anim_t += 1.0 / 60.0;

        let t = ease(camera.anim_t);

        camera.pos = lerp(origin.xy(), target.xy(), t);
        camera.scale = lerp(origin.z, target.z, t);

        if camera.anim_t >= 1.0 {
            camera.pos = target.xy();
            camera.scale = target.z;
            camera.anim_target = None;
        }
    } else {
        if camera.anim_origin.is_some() {
            return;
        }

        if key(keys::A) {
            camera.pos.x += SPEED / camera.scale;
        }

        if key(keys::D) {
            camera.pos.x -= SPEED / camera.scale;
        }

        if key(keys::W) {
            camera.pos.y += SPEED / camera.scale;
        }

        if key(keys::S) {
            camera.pos.y -= SPEED / camera.scale;
        }

        if m.scroll_y != 0 {
            if m.scroll_y > 0 {
                camera.target_scale *= 1.2;
            } else {
                camera.target_scale /= 1.2;
            }

            camera.target_scale = camera.target_scale.clamp(0.002, 1.0);
        }

        // ---

        let scale_delta = camera.target_scale - camera.scale;

        if scale_delta.abs() > 0.001 {
            let world_pos =
                camera.screen_to_world(vec2(m.x as f32, m.y as f32));

            let screen_pos_before = camera.world_to_screen(world_pos);

            camera.scale += scale_delta * 0.25;

            let screen_pos_after = camera.world_to_screen(world_pos);

            camera.pos -= (screen_pos_after - screen_pos_before) / camera.scale;
        }
    }
}

#[derive(Default)]
pub struct Camera {
    pub pos: Vec2,
    pub scale: f32,
    pub target_scale: f32,
    pub anim_target: Option<Vec3>,
    pub anim_origin: Option<Vec3>,
    pub anim_t: f32,
}

impl Camera {
    pub fn size() -> Vec2 {
        vec2(WIDTH as f32, HEIGHT as f32)
    }

    pub fn center() -> Vec2 {
        Self::size() / 2.0
    }

    pub fn contains(&self, pos: Vec2) -> bool {
        pos.x >= 2.0
            && pos.y >= 2.0
            && pos.x <= (WIDTH as f32 - 2.0)
            && pos.y <= (HEIGHT as f32 - 2.0)
    }

    pub fn world_to_screen(&self, pos: Vec2) -> Vec2 {
        Self::center() + (self.pos + pos) * self.scale
    }

    pub fn screen_to_world(&self, pos: Vec2) -> Vec2 {
        (pos - Self::center()) / self.scale - self.pos
    }

    pub fn animate_to(&mut self, target: Vec3) {
        let target_pos = -Self::center() - target.xy();

        self.anim_origin = Some(self.pos.extend(self.scale));
        self.anim_target = Some(target_pos.extend(target.z));
        self.anim_t = 0.0;
    }

    pub fn animate_back(&mut self) {
        let origin = self.anim_origin.take().unwrap();

        self.pos = origin.xy();
        self.scale = origin.z;
        self.anim_target = None;
    }

    pub fn is_animating(&self) -> bool {
        self.anim_target.is_some()
    }
}
