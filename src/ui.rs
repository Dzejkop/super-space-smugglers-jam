use crate::prelude::*;

static mut MOUSE_LEFT_PREV: bool = false;

pub fn tic(game: &mut Game) {
    let m = mouse();
    let mx = m.x as i32;
    let my = m.y as i32;

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
        WIDTH - (16 * 3) - 4,
        HEIGHT - 16 - 4,
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
        WIDTH - (16 * 2) - 4,
        HEIGHT - 16 - 4,
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
        WIDTH - (16 * 1) - 4,
        HEIGHT - 16 - 4,
        SpriteOptions {
            w: 2,
            h: 2,
            transparent: &[0],
            ..Default::default()
        },
    );

    Text::new(format!("Day: {}", game.day()))
        .at(vec2(0.0, 0.0))
        .draw();

    Text::new(format!("Fuel: {}%", game.fuel()))
        .at(vec2(0.0, 8.0))
        .draw();

    Text::new(format!("Money: {}", game.money_str()))
        .at(vec2(0.0, 16.0))
        .draw();

    Text::new(format!("Tickets: {}", game.tickets()))
        .at(vec2(0.0, 24.0))
        .draw();

    // ---

    if m.left && !unsafe { MOUSE_LEFT_PREV } {
        if mouse_over_stop_button {
            game.speed = GameSpeed::Stop;
        } else if mouse_over_play_button {
            game.speed = GameSpeed::Normal;
        } else if mouse_over_fast_button {
            game.speed = GameSpeed::Fast;
        }
    }

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

    unsafe {
        MOUSE_LEFT_PREV = m.left;
    }
}
