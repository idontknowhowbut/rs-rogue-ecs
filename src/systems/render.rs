use crate::{components::{Position, Renderable}, map::{Map, TileType}};
use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};
use hecs::World;
use std::{io::{Stdout}};
use std::io::Write;

pub fn render(stdout: &mut Stdout, world: &World, map: &Map) -> std::io::Result<()> {
    

    execute!(stdout, terminal::Clear(ClearType::All))?;

    render_map(stdout, map);
    render_entities(stdout, world);

     _ = stdout.flush();


    Ok(())
}

fn render_entities(stdout: &mut Stdout, world: &World) {
    for (_, (pos, render)) in world.query::<(&Position, &Renderable)>().iter() {
        let _ = execute!(stdout, cursor::MoveTo(pos.x as u16, pos.y as u16));
        print!("{}", render.glyph);
    }
}

fn render_map(stdout: &mut Stdout, map: &Map) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for tile in &map.map_field {
        let glyph: char;
        match *tile {
            TileType::Floor => glyph = '.',
            TileType::Wall => glyph = '#',
        }
        _ = execute!(stdout, cursor::MoveTo(x as u16, y as u16));
        print!("{}", glyph);
        
        x += 1;
        if x == map.map_width {
            y += 1;
            x = 0;
        }
    }
}
