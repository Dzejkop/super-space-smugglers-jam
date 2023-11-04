use crate::prelude::*;

static mut SHIP: Ship = Ship {
    pos: vec2(1800.0, 0.0),
    vel: vec2(-1.0, 1.0),
    in_orbit: None,
};

pub unsafe fn get() -> &'static Ship {
    unsafe { &SHIP }
}

pub unsafe fn get_mut() -> &'static mut Ship {
    unsafe { &mut SHIP }
}

pub fn tic(camera: &Camera, game: &Game) {
    let ship = unsafe { &mut SHIP };

    let at = camera.world_to_screen(ship.pos);
    let rot = PI - ship.vel.angle_between(Vec2::Y);

    let engine_at = ShipSprite::player()
        .at(at)
        .rot(rot)
        .scale(camera.zoom.max(0.3))
        .engine(true)
        .draw(Some(game));

    if camera.zoom < 0.15 && time() % 1000.0 < 500.0 && !game.manouver_mode {
        circb(at.x as i32, at.y as i32, 8, 5);
    }

    for _ in 0..game.steps() {
        particles::spawn_exhaust(camera.screen_to_world(engine_at), -ship.vel);
    }

    OverflowIndicator::player(at).draw();
}
