mod components;
mod systems;

use components::{Player, Position, Renderable};
use crossterm::{cursor, execute, terminal};
use hecs::World;
use std::io::stdout;

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut world = World::new();
    world.spawn((Position { x: 10, y: 10 }, Renderable { glyph: 'Ö' }, Player));

    loop {
        systems::render::render(&mut stdout, &world)?
    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
