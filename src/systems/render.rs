use crate::components::{Position, Renderable};
use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};
use hecs::World;
use std::io::Stdout;
use std::io::Write;

pub fn render(stdout: &mut Stdout, world: &World) -> std::io::Result<()> {
    execute!(stdout, terminal::Clear(ClearType::All))?;

    for (_, (pos, render)) in world.query::<(&Position, &Renderable)>().iter() {
        execute!(stdout, cursor::MoveTo(pos.x as u16, pos.y as u16))?;
        print!("{}", render.glyph);
    }

    stdout.flush()?;

    Ok(())
}
