use hecs::World;

use crate::{
    components::{Player, Position},
    map::{idx_map, Map, TileType},
};

pub fn process_player_movement(world: &mut World, dx: i32, dy: i32, map: &Map) {
    let mut new_x: i32;
    let mut new_y: i32;
    for (_entity, (_player, pos)) in world.query_mut::<(&Player, &mut Position)>() {
        new_x = pos.x + dx;
        new_y = pos.y + dy;
        let target_tile = &map.map_field[idx_map(new_x, new_y, map.map_width)];
        match target_tile {
            TileType::Floor => {
                pos.x = new_x;
                pos.y = new_y;
            }
            TileType::Wall => return,
        }
    }
}
