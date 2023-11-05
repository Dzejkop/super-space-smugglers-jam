use crate::prelude::*;

pub fn tic(player: &Player, planets: &[Planet]) -> bool {
    planets[0].collides_with(player.ship.pos)
}
