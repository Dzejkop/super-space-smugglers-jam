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

    draw_ship(vec2(32.0, 32.0), time() * 0.001);
}

fn draw_ship(at: Vec2, rot: f32) {
    // Main ship
    Img::sprite(uvec2(1, 2), uvec2(2, 2)).at(at).rot(rot).draw();

    // Bottom thruster
    Img::sprite(uvec2(1, 4), uvec2(2, 2))
        .at(rotate(at + vec2(0.0, 16.0), at, rot))
        .rot(rot)
        .draw();
}
