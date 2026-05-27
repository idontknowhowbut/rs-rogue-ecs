mod components;
mod render;
mod input;

use crossterm::{
    cursor, execute,
    terminal::{self},
};
use hecs::World;
use std::io::stdout;
use components::{Position, Renderable, Player};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut world = World::new();
    world.spawn((Position { x: 10, y: 10 }, Renderable { glyph: '@' }, Player));

    loop {
        render::render(&mut stdout, &world)?;

        if !input::read_input(&mut world)? {
            break;
        }


    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}