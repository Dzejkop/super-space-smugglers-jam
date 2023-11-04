use crate::prelude::mouse;

struct State {
    left_prev: bool,
    right_prev: bool,
}

static mut STATE: State = State {
    left_prev: false,
    right_prev: false,
};

pub fn tic() {
    let mouse = mouse();

    unsafe {
        STATE.left_prev = mouse.left;
        STATE.right_prev = mouse.right;
    }
}

pub fn mouse_left_pressed() -> bool {
    let mouse = mouse();

    unsafe { !STATE.left_prev && mouse.left }
}

pub fn mouse_right_pressed() -> bool {
    let mouse = mouse();

    unsafe { !STATE.right_prev && mouse.right }
}
