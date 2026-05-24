use hecs::{World};

use crate::components::{Player, Position};

pub fn process_player_movement(world: &mut World, dx: i32, dy: i32) {
    if let Some(n) = world.query_mut()::<(&mut Position, &Player)>  {

    };
}
