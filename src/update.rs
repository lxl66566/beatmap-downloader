use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.go_back(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.go_back()
            }
        }
        KeyCode::Left => app.prev_session(),
        KeyCode::Right => app.next_session(),
        KeyCode::Up | KeyCode::Down | KeyCode::Enter => update_app_session(app, key_event.code),
        KeyCode::Char('h') => app.help(),
        _ => {}
    };
}

pub fn update_app_session(app: &mut App, key: KeyCode) {
    match app.session {
        0 => match key {
            KeyCode::Up => app.mode.prev(),
            KeyCode::Down => app.mode.next(),
            _ => {}
        },
        1 => match key {
            KeyCode::Down => app.item.next(),
            KeyCode::Up => app.item.prev(),
            KeyCode::Enter => {
                app.item.select_cursor();
            }
            _ => {}
        },
        _ => unreachable!("session numeber exceeded."),
    }
}
