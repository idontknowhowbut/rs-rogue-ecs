use crate::components::Position;
use crossterm::event::{self, Event, KeyCode};
use hecs::World;

pub fn read_input(world: &mut World) -> std::io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        let mut dx = 0i32;
        let mut dy = 0i32;
        match key.code {
            KeyCode::Left | KeyCode::Char('a') => dx = -1,
            KeyCode::Right | KeyCode::Char('d') => dx = 1,
            KeyCode::Up | KeyCode::Char('w') => dy = -1,
            KeyCode::Down | KeyCode::Char('s') => dy = 1,
            KeyCode::Char('q') => return Ok(false),
            _ => {}
        }
        for (_, pos) in world.query::<&mut Position>().iter() {
            pos.x += dx;
            pos.y += dy;
        }
    }

    Ok(true)
}
