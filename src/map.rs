use fastrand;

#[derive(Clone)]
pub enum TileType{
    Wall, Floor
}


pub fn new_map(x_size: i32, y_size: i32) -> Vec<TileType> {
    let map_size: usize = (x_size * y_size) as usize;
    let mut map = vec![TileType::Floor; map_size];
    for i in 0..x_size {
        map[idx_map(i, 0, x_size)] = TileType::Wall;
        map[idx_map(i, y_size, x_size)] = TileType::Wall
    }

    for i in 0..y_size {
        map[idx_map(0, i, x_size)] = TileType::Wall;
        map[idx_map(x_size, i, x_size)] = TileType::Wall
    }

    //shiid loop to fill 10% of map with walls at random. Replace with proper mapgen
    for _ in 0..x_size*y_size/100 {  
        let r = fastrand::usize(..map.len());
        map[r] = TileType::Wall;
    }
    
    return map
}

fn idx_map(x: i32, y: i32, x_size: i32) -> usize {
    ((y * x_size) + x) as usize
}

