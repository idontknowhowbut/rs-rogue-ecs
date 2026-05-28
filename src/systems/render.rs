use crate::{
    components::{Position, Renderable},
    map::{Map, TileType},
};
use crossterm::{
    cursor, queue,
};
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
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut map_row: Vec<char> = Vec::new();

    for tile in &map.map_field[x as usize..(x+map.map_width*y) as usize] {


        let glyph: char;
        match *tile {
            TileType::Floor => glyph = '.',
            TileType::Wall => glyph = '#',
        }

        x += 1;
        if x == map.map_width {
            y += 1;
            x = 0;
        }
    }
}
