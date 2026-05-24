mod components;
mod systems;

use components::{Player, Position, Renderable};
use crossterm::{
    cursor,
    execute, terminal,
};
use hecs::World;
use std::io::stdout;

use crate::systems::{
    input::{scan_input, Intent},
    movement,
};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut world = World::new();
    world.spawn((Position { x: 10, y: 10 }, Renderable { glyph: 'Ö' }, Player));

    loop {
        systems::render::render(&mut stdout, &world)?;
        let intent: systems::input::Intent = scan_input();
        match intent {
            Intent::Move { dx, dy } => 
                movement::process_player_movement(&mut world, dx, dy),
            Intent::Quit => break,
            Intent::None => (),
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
