use crate::prelude::*;
use crate::screen_shake::add_shake;

const MAX_MANOUVER_LENGTH: f32 = 10.0;

pub fn tic(
    camera: &Camera,
    game: &mut Game,
    player: &mut Player,
    planets: &[Planet],
) {
    if !game.is_paused() {
        return;
    }

    let mouse = mouse();
    let mouse_xy = vec2(mouse.x as f32, mouse.y as f32);

    let vec = camera.world_to_screen(player.ship.pos) - mouse_xy;
    let dist = vec.length();

    game.manouver_dv = {
        let manouver_dir = vec.normalize();
        let manouver_len = vec.length();
        let manouver_sensitivity = lerp(0.02, 0.08, manouver_len / 64.0);

        let manouver = manouver_dir * manouver_len * manouver_sensitivity;
        let max_manouver_len = game.fuel * MAX_MANOUVER_LENGTH;

        if max_manouver_len == 0.0 {
            vec2(0.0, 0.0)
        } else if manouver.length() <= max_manouver_len {
            manouver
        } else {
            manouver.normalize() * max_manouver_len
        }
    };

    game.manouver_fuel =
        (game.manouver_dv.length() / MAX_MANOUVER_LENGTH).max(0.04);

    if !game.manouver_mode && dist < 10.0 {
        SelectionIndicator::new(camera.world_to_screen(player.ship.pos))
            .size(vec2(16.0, 16.0))
            .draw();

        game.manouver_mode |= mouse.left;
    }

    if game.manouver_mode && (mouse.right || key(keys::X)) {
        game.manouver_mode = false;
    }

    if game.manouver_mode && !mouse.left {
        game.manouver_mode = false;

        if game.manouver_dv.length() > 0.0 {
            if game.fuel <= 0.00001 {
                msgs::add("You don't have fuel.");
            } else {
                player.ship.vel += game.manouver_dv;
                game.fuel -= game.manouver_fuel;

                if game.fuel < 0.01 {
                    game.fuel = 0.0;
                }

                add_shake();

                sfx(
                    4,
                    SfxOptions {
                        note: 0,
                        octave: 3,
                        duration: 20,
                        volume_left: 8,
                        volume_right: 8,
                        ..Default::default()
                    },
                );

                game.speed = GameSpeed::Normal;
            }
        }
    }

    if game.manouver_mode && game.manouver_dv.length() > 0.0 {
        let mut player = player.ship;

        player.vel += game.manouver_dv;

        let mut prev_step = player.pos;
        let steps = sim::trajectory(game, &player, planets).take(750);

        for step in steps {
            let p1 = camera.world_to_screen(prev_step);
            let p2 = camera.world_to_screen(step.pos);

            let display = if step.touches {
                time() % 500.0 < 250.0
            } else {
                true
            };

            if display {
                line(p1.x, p1.y, p2.x, p2.y, step.color);
            }

            prev_step = step.pos;
        }
    }
}
