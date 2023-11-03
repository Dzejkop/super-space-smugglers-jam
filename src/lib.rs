mod alloc;
mod tic80;
mod utils;

mod btns {
    pub const UP: i32 = 0;
    pub const DOWN: i32 = 1;
    pub const LEFT: i32 = 2;
    pub const RIGHT: i32 = 3;
}

use glam::*;
use tic80::*;
use utils::*;

#[export_name = "TIC"]
pub fn tic() {
    cls(0);

    Sprite::slice(uvec2(1, 0), uvec2(2, 1))
        .at(vec2(32.0, 32.0))
        .rot(time() * 0.01)
        .scale(2.0)
        .render();
}
