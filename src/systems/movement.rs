use hecs::World;

<<<<<<< HEAD
<<<<<<< HEAD
use crate::components::{Player, Position};

pub fn process_player_movement(world: &mut World, dx: i32, dy: i32) {
    for (_entity, (_player, pos)) in world.query_mut::<(&Player, &mut Position)>() {
        pos.x += dx;
        pos.y += dy;
    }
}
=======
pub fn process_movement(world: &mut World, dx: i32, dy: i32) {}
>>>>>>> 2e8e1a0 (develop: refactor input, add basic movement sys)
=======
use crate::components::{Player, Position};

pub fn process_player_movement(world: &mut World, dx: i32, dy: i32) {
    for (_entity, (_player, pos)) in world.query_mut::<(&Player, &mut Position)>() {
        pos.x += dx;
        pos.y += dy;
    }
}
>>>>>>> ed5b3f5 (develop: iterate movement)
