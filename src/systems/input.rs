use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
pub enum Intent {
    Quit,
    Move { dx: i32, dy: i32 },
    None,
}

pub fn scan_input() -> Intent {
    let mut result: Intent = Intent::None;
    if poll(Duration::from_millis(100)).unwrap_or(false) {
        if let Ok(Event::Key(key)) = read() {
            match key.code {
                KeyCode::Char('q') => result = Intent::Quit,
                KeyCode::Char('a') => result = Intent::Move { dx: -1, dy: 0 },
                KeyCode::Char('d') => result = Intent::Move { dx: 1, dy: 0 },
                KeyCode::Char('w') => result = Intent::Move { dx: 0, dy: -1 },
                KeyCode::Char('s') => result = Intent::Move { dx: 0, dy: 1 },
                _ => (),
            }
        }
    }
    return result;
}
