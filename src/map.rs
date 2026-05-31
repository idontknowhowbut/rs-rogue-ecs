#[derive(Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub map_field: Vec<TileType>,
    pub map_width: i32,
    pub map_height: i32,
}

pub fn new_map(x_size: i32, y_size: i32) -> Map {
    let map_size: usize = (x_size * y_size) as usize;
    let mut map_field = vec![TileType::Floor; map_size];
  
    map_field = crate::mapgen::generate_cave(x_size, y_size);    

    let mut map = Map {
        map_field: map_field,
        map_width: x_size,
        map_height: y_size,
    };

    border_map(&mut map);
    return map;
}

pub fn idx_map(x: i32, y: i32, x_size: i32) -> usize {
    ((y * x_size) + x) as usize
}

fn border_map(map: &mut Map) {
    let x_size: i32 = map.map_width;
    let y_size: i32 = map.map_height;
    for i in 0..x_size {
        map.map_field[idx_map(i, 0, x_size)] = TileType::Wall;
        map.map_field[idx_map(i, y_size - 1, x_size)] = TileType::Wall
    }

    for i in 0..y_size {
        map.map_field[idx_map(0, i, x_size)] = TileType::Wall;
        map.map_field[idx_map(x_size - 1, i, x_size)] = TileType::Wall
    }
}
