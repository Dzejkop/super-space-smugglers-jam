use std::collections::VecDeque;
use std::fmt::Display;

use crate::prelude::*;

static mut MSGS: VecDeque<String> = VecDeque::new();
static mut MSG: Option<Message> = None;

pub fn tic(game: &Game) {
    if game.manouver_mode {
        return;
    }

    let msgs = unsafe { &mut MSGS };
    let msg = unsafe { &mut MSG };

    if msg.is_none() {
        *msg = msgs.pop_front().map(|text| Message {
            text,
            offset: 0.0,
            anim: MessageAnimation::GoingDown,
        });
    }

    let completed = if let Some(msg) = msg {
        if let MessageAnimation::GoingDown
        | MessageAnimation::Stationary { .. }
        | MessageAnimation::GoingUp = &msg.anim
        {
            Text::new(&msg.text)
                .at(vec2(0.0, -8.0) + vec2(0.0, msg.offset))
                .draw();
        }

        match &mut msg.anim {
            MessageAnimation::GoingDown => {
                msg.offset += 0.33;

                if msg.offset >= 10.0 {
                    msg.anim = MessageAnimation::Stationary { elapsed: 0.0 };
                }

                false
            }

            MessageAnimation::Stationary { elapsed } => {
                *elapsed += 1.0;

                if *elapsed > 90.0 {
                    msg.anim = MessageAnimation::GoingUp;
                }

                false
            }

            MessageAnimation::GoingUp => {
                if msgs.is_empty() {
                    msg.offset -= 0.66;
                } else {
                    msg.offset -= 1.0;
                }

                if msg.offset <= 0.0 {
                    msg.anim = MessageAnimation::Completed;
                }

                false
            }

            MessageAnimation::Completed => true,
        }
    } else {
        false
    };

    if completed {
        *msg = None;
    }
}

pub fn add(msg: impl Display) {
    unsafe {
        MSGS.push_back(msg.to_string());
    }
}

struct Message {
    text: String,
    offset: f32,
    anim: MessageAnimation,
}

enum MessageAnimation {
    GoingDown,
    Stationary { elapsed: f32 },
    GoingUp,
    Completed,
}
