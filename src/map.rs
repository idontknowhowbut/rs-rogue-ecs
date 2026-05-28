use fastrand;

#[derive(Clone)]
pub enum TileType{
    Wall, Floor
}

pub struct Map {
    pub map_field: Vec<TileType>,
    pub map_width: i32
}


pub fn new_map(x_size: i32, y_size: i32) -> Map {
    let map_size: usize = (x_size * y_size) as usize;
    let mut map_field = vec![TileType::Floor; map_size];
    for i in 0..x_size {
        map_field[idx_map(i, 0, x_size)] = TileType::Wall;
        map_field[idx_map(i, y_size - 1, x_size)] = TileType::Wall
    }

    for i in 0..y_size {
        map_field[idx_map(0, i, x_size)] = TileType::Wall;
        map_field[idx_map(x_size - 1, i, x_size)] = TileType::Wall
    }

    //shiid loop to fill 10% of map with walls at random. Replace with proper mapgen
    for _ in 0..x_size*y_size/10 {  
        let r = fastrand::usize(..map_field.len());
        map_field[r] = TileType::Wall;
    }


    let map= Map {
        map_field: map_field,
        map_width: x_size
    };

    return map
}

fn idx_map(x: i32, y: i32, x_size: i32) -> usize {
    ((y * x_size) + x) as usize
}

