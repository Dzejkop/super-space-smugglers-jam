use crate::prelude::*;

pub struct Player {
    pub ship: Ship,
    pub is_spawned: bool,
    pub is_just_spawned: bool,
    pub is_caught: bool,
}

static mut PLAYER: Player = Player {
    ship: Ship {
        pos: vec2(1800.0, 0.0),
        vel: vec2(-1.0, 1.0),
    },
    is_spawned: false,
    is_just_spawned: false,
    is_caught: false,
};

pub unsafe fn get() -> &'static Player {
    unsafe { &PLAYER }
}

pub unsafe fn get_mut() -> &'static mut Player {
    unsafe { &mut PLAYER }
}

pub fn tic(camera: &Camera, game: &Game) -> bool {
    let player = unsafe { &mut PLAYER };

    // ---

    if !player.is_spawned {
        Text::new("Choose your spawn place.")
            .at(vec2(0.0, 0.0))
            .draw();

        Text::new("Use WASD & scroll to move camera; click to")
            .at(vec2(0.0, 16.0))
            .draw();

        Text::new("spawn.").at(vec2(0.0, 24.0)).draw();

        player.ship.pos = camera.screen_to_world(mouse_pos());
    }

    // ---

    let at = camera.world_to_screen(player.ship.pos);

    let rot = if player.is_spawned {
        PI - player.ship.vel.angle_between(Vec2::Y)
    } else {
        time() / 250.0
    };

    let min_scale = if player.is_spawned { 0.3 } else { 0.6 };

    let engine_at = ShipSprite::player()
        .at(at)
        .rot(rot)
        .scale(camera.scale.max(min_scale))
        .engine(player.is_spawned)
        .draw(Some(game));

    if player.is_spawned {
        for _ in 0..game.steps() {
            particles::spawn_exhaust(
                camera.screen_to_world(engine_at),
                -player.ship.vel,
            );
        }
    }

    // ---

    if !player.is_caught {
        Localizator::player(at).draw();
    }

    if player.is_just_spawned {
        player.is_just_spawned = false;
    }

    if !player.is_spawned && mouse().left {
        let rot = rot - PI / 2.0;

        player.is_spawned = true;
        player.is_just_spawned = true;
        player.ship.vel = vec2(rot.cos(), rot.sin());

        msgs::add("Good luck!");

        true
    } else {
        false
    }
}
