use crate::prelude::*;

pub fn tic(game: &mut Game, police: &police::State) {
    let m = mouse();
    let mx = m.x as i32;
    let my = m.y as i32;

    // -- Money --
    if !game.manouver_mode {
        Text::new(format!("${}k", game.money))
            .at(vec2(WIDTH as f32, 0.0))
            .align_right()
            .draw();
    }

    // -- Police stars --
    if !game.manouver_mode {
        for idx in 0..3 {
            let active = match idx {
                0 => police.wanted() > 0.0,
                1 => police.wanted() >= 0.4,
                2 => police.wanted() >= 0.8,
                _ => unreachable!(),
            };

            let almost_inactive = match idx {
                0 => (0.0..=0.1).contains(&police.wanted()),
                1 => (0.4..=0.5).contains(&police.wanted()),
                2 => (0.8..=0.9).contains(&police.wanted()),
                _ => unreachable!(),
            };

            let sprite = if active {
                if almost_inactive {
                    if game.time() % 1000.0 < 500.0 {
                        263
                    } else {
                        262
                    }
                } else {
                    263
                }
            } else {
                262
            };

            Img::sprite_idx(sprite)
                .at(vec2(WIDTH as f32 - 4.0 - 20.0 + 10.0 * (idx as f32), 13.0))
                .scale(1.0)
                .draw();
        }
    }

    // -- Time controls --
    if !game.manouver_mode {
        let mouse_over_stop_button = mx >= WIDTH - (16 * 3) - 4
            && mx < WIDTH - (16 * 4) - 4 + 16 * 2
            && my >= HEIGHT - 16 - 4
            && my < HEIGHT - 16 - 4 + 16 * 2;

        let mouse_over_play_button = mx >= WIDTH - (16 * 2) - 4
            && mx < WIDTH - (16 * 3) - 4 + 16 * 2
            && my >= HEIGHT - 16 - 4
            && my < HEIGHT - 16 - 4 + 16 * 2;

        let mouse_over_fast_button = mx >= WIDTH - (16 * 1) - 4
            && mx < WIDTH - (16 * 2) - 4 + 16 * 2
            && my >= HEIGHT - 16 - 4
            && my < HEIGHT - 16 - 4 + 16 * 2;

        spr(
            if matches!(game.speed, GameSpeed::Stop) {
                sprites::buttons::active::STOP
            } else if mouse_over_stop_button {
                sprites::buttons::highlighted::STOP
            } else {
                sprites::buttons::inactive::STOP
            },
            WIDTH - 16 * 3,
            HEIGHT - 16,
            SpriteOptions {
                w: 2,
                h: 2,
                transparent: &[0],
                ..Default::default()
            },
        );

        spr(
            if matches!(game.speed, GameSpeed::Normal) {
                sprites::buttons::active::NORMAL
            } else if mouse_over_play_button {
                sprites::buttons::highlighted::NORMAL
            } else {
                sprites::buttons::inactive::NORMAL
            },
            WIDTH - 16 * 2,
            HEIGHT - 16,
            SpriteOptions {
                w: 2,
                h: 2,
                transparent: &[0],
                ..Default::default()
            },
        );

        spr(
            if matches!(game.speed, GameSpeed::Fast) {
                sprites::buttons::active::FAST
            } else if mouse_over_fast_button {
                sprites::buttons::highlighted::FAST
            } else {
                sprites::buttons::inactive::FAST
            },
            WIDTH - 16,
            HEIGHT - 16,
            SpriteOptions {
                w: 2,
                h: 2,
                transparent: &[0],
                ..Default::default()
            },
        );

        if mouse_left_pressed() {
            if mouse_over_stop_button {
                game.speed = GameSpeed::Stop;
            } else if mouse_over_play_button {
                game.speed = GameSpeed::Normal;
            } else if mouse_over_fast_button {
                game.speed = GameSpeed::Fast;
            }
        }
    }

    // -- Fuel UI --
    let show_fuel =
        if game.manouver_mode && (game.fuel - game.manouver_fuel) < 0.01 {
            time() % 1000.0 < 500.0
        } else {
            true
        };

    if show_fuel {
        let fuel_height = 3.0 * 16.0 - 6.0;

        let fuel_h = (game.fuel() * fuel_height) as i32;
        let fuel_y = HEIGHT - fuel_h - 2;

        rect(2, fuel_y, 12, fuel_h, 6);

        if game.manouver_mode {
            let fuel_cost_h = (game.manouver_fuel * fuel_height) as i32;

            rect(2, fuel_y, 12, fuel_cost_h, 2);
        }

        spr(
            14,
            0,
            HEIGHT - 16 * 3,
            SpriteOptions {
                w: 2,
                h: 6,
                transparent: &[0],
                ..Default::default()
            },
        );
    }

    // -- Keyboard controls --
    if !game.manouver_mode {
        if key(keys::DIGIT_1) {
            game.speed = GameSpeed::Stop;
        } else if key(keys::DIGIT_2) {
            game.speed = GameSpeed::Normal;
        } else if key(keys::DIGIT_3) {
            game.speed = GameSpeed::Fast;
        } else if keyp(keys::SPACE, 16, 16) {
            game.speed = match game.speed {
                GameSpeed::Stop => GameSpeed::Normal,
                _ => GameSpeed::Stop,
            };
        }
    }

    // ---

    if game.manouver_mode {
        if game.fuel == 0.0 {
            Text::new("You don't have fuel.")
                .at(vec2(0.0, 0.0))
                .color(2)
                .draw();
        } else {
            Text::new("Release left mouse button to confirm.")
                .at(vec2(0.0, 0.0))
                .color(14)
                .draw();

            Text::new("Press right mouse button or X to cancel.")
                .at(vec2(0.0, 8.0))
                .color(14)
                .draw();
        }
    }

    // ---
}
