use crate::prelude::*;

static mut SHAKE: f32 = 0.0;
const MAG: i8 = 4;

pub fn tic(rng: &mut dyn RngCore) {
    unsafe {
        if SHAKE > 0.0 {
            SHAKE -= 1.0;

            let shake_x = rng.gen::<i8>() % MAG;
            let shake_y = rng.gen::<i8>() % MAG;

            poke(0x3FF9, shake_x as u8);
            poke(0x3FFA, shake_y as u8);
        }

        if SHAKE == 0.0 {
            memset(0x3FF9, 0, 2);
        }
    }
}

pub fn add_shake() {
    unsafe {
        SHAKE = 20.0;
    }
}
