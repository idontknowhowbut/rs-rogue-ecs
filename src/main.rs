mod components;
mod systems;
mod map;

use components::{Player, Position, Renderable};
use crossterm::{
    cursor,
    execute, terminal,
};
use hecs::World;
use std::{io::stdout};
use crate::systems::{
    input::{scan_input, Intent},
    movement,
};

const MAP_HEIGHT: i32 = 50;
const MAP_WIDTH: i32 = 80;


use crate::systems::{
    input::{scan_input, Intent},
    movement,
};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    let mut world = World::new();
    world.spawn((Position { x: 10, y: 10 }, Renderable { glyph: 'Ö' }, Player));

    let map = map::new_map(MAP_WIDTH, MAP_HEIGHT);

    loop {
        systems::render::render(&mut stdout, &world, &map)?;
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


