use crate::{
    components::{Position, Renderable},
    map::{Map, TileType},
};
use crossterm::{cursor, queue};
use hecs::World;
use std::io::Stdout;
use std::io::Write;

pub fn render(stdout: &mut Stdout, world: &World, map: &Map) -> std::io::Result<()> {
    render_map(stdout, map);
    render_entities(stdout, world);

    _ = stdout.flush();
    Ok(())
}

fn render_entities(stdout: &mut Stdout, world: &World) {
    for (_, (pos, render)) in world.query::<(&Position, &Renderable)>().iter() {
        let _ = queue!(stdout, cursor::MoveTo(pos.x as u16, pos.y as u16));
        _ = write!(stdout, "{}", render.glyph);
    }
}

fn render_map(stdout: &mut Stdout, map: &Map) {
    let mut y: i32 = 0;

    while map.map_width + map.map_width * y <= map.map_field.len() as i32 {
        let mut map_row: Vec<char> = Vec::new();

        for tile in &map.map_field
            [(map.map_width * y) as usize..(map.map_width + map.map_width * y) as usize]
        {
            match *tile {
                TileType::Floor => map_row.push('.'),
                TileType::Wall => map_row.push('#'),
            }

        }

        let s: String = map_row.into_iter().collect();
        let _ = queue!(stdout, cursor::MoveTo(0, y as u16));
        _ = write!(stdout, "{}", s);
        y += 1;
    }
}
