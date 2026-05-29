use crate::{
    components::{Position, Renderable},
    map::{Map, TileType},
};
use crossterm::{cursor, execute, queue, style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor}};
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
        _ = queue!(stdout, SetForegroundColor(Color::DarkGreen), cursor::MoveTo(pos.x as u16, pos.y as u16));
        _ = write!(stdout, "{}", render.glyph);
        _ = execute!(stdout, ResetColor);
    }
    
}

fn render_map(stdout: &mut Stdout, map: &Map) {
    let mut y: i32 = 0;

    while y < map.map_height {
        let mut map_row: Vec<char> = Vec::new();

        let row_start = (map.map_width * y) as usize;
        let row_end = row_start + map.map_width as usize;
        for tile in &map.map_field[row_start..row_end] {
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
