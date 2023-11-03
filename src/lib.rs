mod alloc;
mod tic80;
mod utils;

use tic80::*;
use utils::*;

mod sprites {}

mod btns {
    pub const UP: i32 = 0;
    pub const DOWN: i32 = 1;
    pub const LEFT: i32 = 2;
    pub const RIGHT: i32 = 3;
}

#[export_name = "TIC"]
pub fn tic() {
    cls(0);
}
