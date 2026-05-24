use hecs::{World};

use crate::components::{Player, Position};

pub fn process_player_movement(world: &mut World, dx: i32, dy: i32) {
    for (_entity, (_player, pos)) in world.query_mut::<(&Player, &mut Position)>() {
        pos.x += dx;
        pos.y += dy;
    }
}
