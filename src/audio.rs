use std::collections::VecDeque;

use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum Note {
    Play {
        sfx: u8,
        note: u8,
        octave: u8,
        duration: u16,
    },
    Wait {
        duration: u16,
    },
}

static mut QUEUE: VecDeque<Note> = VecDeque::new();
static mut NEXT_IN: u16 = 0;

pub fn tic() {
    let queue = unsafe { &mut QUEUE };
    let next_in = unsafe { &mut NEXT_IN };

    *next_in = next_in.saturating_sub(1);

    while *next_in == 0 {
        match queue.pop_front() {
            Some(Note::Play {
                sfx,
                note,
                octave,
                duration,
            }) => crate::tic80::sfx(
                sfx as i32,
                SfxOptions {
                    note: note as i32,
                    octave: octave as i32,
                    duration: duration as i32,
                    channel: 1,
                    volume_left: 15,
                    volume_right: 15,
                    speed: 0,
                },
            ),

            Some(Note::Wait { duration }) => {
                *next_in = duration;
                break;
            }

            None => {
                break;
            }
        }
    }
}

pub fn play(notes: &[Note]) {
    unsafe {
        QUEUE.extend(notes.iter().copied());
    }
}
