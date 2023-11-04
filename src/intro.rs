use crate::prelude::*;

static mut STATE: State = State::AwaitingAnyKey;

enum State {
    AwaitingAnyKey,

    AnimatingOut {
        ship_positions: Vec<Vec2>,
        ship_velocities: Vec<Vec2>,
        elapsed: f32,
    },
}

pub fn tic() -> bool {
    let state = unsafe { &mut STATE };
    let time = time() / 1000.0;

    // ---

    let text_offset = match state {
        State::AwaitingAnyKey => 0.0,
        State::AnimatingOut { elapsed, .. } => *elapsed,
    };

    Text::new("Space: Out of Control")
        .at(vec2(WIDTH as f32, 45.0 - text_offset))
        .centered()
        .draw();

    Text::new("Game for TK Game Jam 2023")
        .at(vec2(WIDTH as f32, 60.0 - text_offset))
        .centered()
        .draw();

    Text::new("by dzejkop & Patryk27")
        .at(vec2(WIDTH as f32, 68.0 - text_offset))
        .centered()
        .draw();

    if time % 1.0 < 0.5 {
        Text::new("Press any key to start")
            .at(vec2(WIDTH as f32, 90.0 + text_offset))
            .color(5)
            .centered()
            .draw();
    }

    // ---

    match state {
        State::AwaitingAnyKey => {
            let mut ship_positions = Vec::new();
            let mut ship_velocities = Vec::new();

            // ---

            let player_at = get_player_pos(time);
            let player_vel =
                get_velocity(player_at, get_player_pos(time + 0.01));
            let player_rot = get_rotation(player_vel);

            let player_engine_at = ShipSprite::player()
                .at(player_at)
                .rot(player_rot)
                .engine(true)
                .draw();

            ship_positions.push(player_at);
            ship_velocities.push(player_vel);

            particles::spawn(
                player_engine_at,
                -player_vel.normalize() * 2.5,
                266,
                271,
                22,
            );

            // ---

            for idx in 0..3 {
                let police_at = get_police_pos(idx, time);
                let police_vel =
                    get_velocity(police_at, get_police_pos(idx, time + 0.01));
                let police_rot = get_rotation(police_vel);

                let police_engine_at = ShipSprite::police()
                    .at(police_at)
                    .rot(police_rot)
                    .engine(true)
                    .draw();

                particles::spawn(
                    police_engine_at,
                    -police_vel.normalize() * 2.5,
                    266,
                    271,
                    22,
                );

                ship_positions.push(police_at);
                ship_velocities.push(police_vel);
            }

            // ---

            if any_key() {
                *state = State::AnimatingOut {
                    ship_positions,
                    ship_velocities,
                    elapsed: 0.0,
                };
            }
        }

        State::AnimatingOut {
            ship_positions,
            ship_velocities,
            elapsed,
        } => {
            *elapsed += 0.75;

            for (ship_idx, (ship_pos, ship_vel)) in
                ship_positions.iter_mut().zip(ship_velocities).enumerate()
            {
                *ship_pos += *ship_vel;

                // ---

                let ship = if ship_idx == 0 {
                    ShipSprite::player()
                } else {
                    ShipSprite::police()
                };

                let ship_engine_at = ship
                    .at(*ship_pos)
                    .rot(get_rotation(*ship_vel))
                    .engine(true)
                    .draw();

                particles::spawn(
                    ship_engine_at,
                    -*ship_vel * 2.5,
                    266,
                    271,
                    22,
                );
            }

            if *elapsed > 12.0 && !particles::is_any_visible() {
                return true;
            }
        }
    }

    false
}

fn centerish() -> Vec2 {
    vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0 - 10.0)
}

fn get_player_pos(time: f32) -> Vec2 {
    centerish() + vec2(time.cos() * 100.0, time.sin() * 60.0)
}

fn get_police_pos(idx: usize, time: f32) -> Vec2 {
    let time = match idx {
        0 => time - 1.5,
        1 => time - 2.0,
        2 => time - 2.2,
        _ => unreachable!(),
    };

    let f = match idx {
        0 => 1.0,
        1 => 0.9,
        2 => 1.1,
        _ => unreachable!(),
    };

    centerish() + vec2(time.cos() * 100.0 * f, time.sin() * 50.0 * f)
}

fn get_velocity(p1: Vec2, p2: Vec2) -> Vec2 {
    (p2 - p1) * 1.9
}

fn get_rotation(vel: Vec2) -> f32 {
    PI - vel.normalize().angle_between(Vec2::Y)
}

fn any_key() -> bool {
    for i in 0..32 {
        if btn(i) {
            return true;
        }
    }

    for i in 0..65 {
        if key(i) {
            return true;
        }
    }

    false
}
