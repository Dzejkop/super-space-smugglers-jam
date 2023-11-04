use crate::prelude::*;

static mut SHIP: Ship = Ship {
    pos: vec2(175.0, 0.0),
    vel: vec2(0.0, 1.0),
    in_orbit: None,
};

pub unsafe fn get() -> &'static Ship {
    unsafe { &SHIP }
}

pub unsafe fn get_mut() -> &'static mut Ship {
    unsafe { &mut SHIP }
}

pub fn tic(camera: &Camera, game: &Game, planets: &[Planet]) {
    let ship = unsafe { &mut SHIP };

    // ---

    if game.dt() != 0.0 {
        for planet in planets {
            let d = planet.pos - ship.pos;
            let d2 = d.length_squared();
            let f = planet.mass / d2;

            ship.vel += f * d * game.dt();
        }

        ship.pos += ship.vel * game.dt();
    }

    // ---

    let at = camera.world_to_screen(ship.pos);

    let engine_at = ShipSprite::player()
        .at(at)
        .rot(game.time() * 0.001)
        .scale((3.0 * camera.zoom).max(0.15))
        .engine(true)
        .draw();

    if camera.zoom < 0.15 && time() % 1000.0 < 500.0 {
        circb(at.x as i32, at.y as i32, 8, 5);
    }

    OverflowIndicator::player(at).draw();
}
