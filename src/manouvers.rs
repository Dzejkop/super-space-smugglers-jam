use crate::prelude::*;

const MAX_MANOUVER_LENGTH: f32 = 10.0;

pub fn tic(
    camera: &Camera,
    game: &mut Game,
    player: &mut Ship,
    planets: &[Planet],
) {
    if !game.is_paused() {
        return;
    }

    let mouse = mouse();
    let mouse_xy = vec2(mouse.x as f32, mouse.y as f32);

    let vec = camera.world_to_screen(player.pos) - mouse_xy;
    let dist = vec.length();

    game.manouver_dv = {
        let manouver_dir = vec.normalize();
        let manouver_len = vec.length();
        let manouver_sensitivity = lerp(0.008, 0.03, manouver_len / 100.0);

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
        SelectionIndicator::new(camera.world_to_screen(player.pos))
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
                player.vel.x += game.manouver_dv.x;
                player.vel.y += game.manouver_dv.y;
                game.fuel -= game.manouver_fuel;

                if game.fuel < 0.01 {
                    game.fuel = 0.0;
                }
            }
        }
    }

    if game.manouver_mode && game.manouver_dv.length() > 0.0 {
        let mut t_ship = *player;

        t_ship.vel.x += game.manouver_dv.x;
        t_ship.vel.y += game.manouver_dv.y;

        let mut prev_step = t_ship.pos;

        let steps =
            orbits::trajectory(game.time(), &t_ship, planets).take(1000);

        for step in steps {
            let p1 = camera.world_to_screen(prev_step);
            let p2 = camera.world_to_screen(vec2(step.x, step.y));

            if step.n > 500 {
                if step.n % 8 == 0 {
                    pix(p1.x as i32, p1.y as i32, 14);
                }
            } else if step.n > 250 {
                if step.n % 4 == 0 {
                    pix(p2.x as i32, p2.y as i32, 12);
                }
            } else {
                line(p1.x, p1.y, p2.x, p2.y, 12);
            }

            prev_step = vec2(step.x, step.y);
        }
    }
}
