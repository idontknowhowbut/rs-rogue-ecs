use crate::map::{idx_map, TileType};
use fastrand;

fn count_wall_neighbors_r1(map: &Vec<TileType>, x: i32, y: i32, width: i32, height: i32) -> i32 {
    let mut count = 0;
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= width || ny >= height {
                count += 1;
            } else {
                if matches!(map[idx_map(nx, ny, width)], TileType::Wall) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn count_wall_neighbors_r2(map: &Vec<TileType>, x: i32, y: i32, width: i32, height: i32) -> i32 {
    let mut count = 0;
    for dy in -2i32..=2 {
        for dx in -2i32..=2 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= width || ny >= height {
                count += 1;
            } else {
                if matches!(map[idx_map(nx, ny, width)], TileType::Wall) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn simulate_step(map: &Vec<TileType>, width: i32, height: i32, use_r2: bool) -> Vec<TileType> {
    let mut new_map = map.clone();
    for y in 0..height {
        for x in 0..width {
            let r1 = count_wall_neighbors_r1(map, x, y, width, height);
            let wall = if use_r2 {
                let r2 = count_wall_neighbors_r2(map, x, y, width, height);
                r1 >= 5 || r2 <= 2
            } else {
                r1 >= 5
            };
            new_map[idx_map(x, y, width)] = if wall {
                TileType::Wall
            } else {
                TileType::Floor
            };
        }
    }
    new_map
}

pub fn generate_cave(width: i32, height: i32) -> Vec<TileType> {
    let size = (width * height) as usize;

    // random %
    let mut map: Vec<TileType> = (0..size)
        .map(|_| {
            if fastrand::f32() < 0.545 {
                TileType::Wall
            } else {
                TileType::Floor
            }
        })
        .collect();

    for _ in 0..2 {
        map = simulate_step(&map, width, height, true); // r1+r2
    }
    for _ in 0..3 {
        map = simulate_step(&map, width, height, false); // r1
    }

    remove_isolated_regions(&mut map, width, height);

    map
}

fn flood_fill(
    map: &Vec<TileType>,
    start_x: i32,
    start_y: i32,
    width: i32,
    height: i32,
) -> Vec<(i32, i32)> {
    use std::collections::VecDeque;

    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    let mut visited = vec![false; (width * height) as usize];
    let mut result: Vec<(i32, i32)> = Vec::new();

    queue.push_back((start_x, start_y));
    visited[idx_map(start_x, start_y, width)] = true;

    while let Some((x, y)) = queue.pop_front() {
        result.push((x, y));

        // 4!! neighbors not 8
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if nx < 0 || ny < 0 || nx >= width || ny >= height {
                continue;
            }
            if visited[idx_map(nx, ny, width)] {
                continue;
            }
            if matches!(map[idx_map(nx, ny, width)], TileType::Wall) {
                continue;
            }

            visited[idx_map(nx, ny, width)] = true;
            queue.push_back((nx, ny));
        }
    }

    result
}

pub fn remove_isolated_regions(map: &mut Vec<TileType>, width: i32, height: i32) {
    let mut visited = vec![false; (width * height) as usize];
    let mut regions: Vec<Vec<(i32, i32)>> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if visited[idx_map(x, y, width)] {
                continue;
            }
            if matches!(map[idx_map(x, y, width)], TileType::Wall) {
                continue;
            }

            let region = flood_fill(map, x, y, width, height);

            for (rx, ry) in &region {
                visited[idx_map(*rx, *ry, width)] = true;
            }

            regions.push(region);
        }
    }

    let largest = regions
        .iter()
        .enumerate()
        .max_by_key(|(_, r)| r.len())
        .map(|(i, _)| i);

    eprintln!("regions found: {}", regions.len());
    for (i, r) in regions.iter().enumerate() {
        eprintln!("  region {}: {} cells", i, r.len());
    }

    if let Some(i) = largest {
        for (idx, region) in regions.iter().enumerate() {
            if idx != i {
                for (x, y) in region {
                    map[idx_map(*x, *y, width)] = TileType::Wall;
                }
            }
        }
    }
}
