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
        KeyCode::Down => app.item.next(),
        KeyCode::Up => app.item.prev(),
        KeyCode::Enter => {
            app.item.select_cursor();
        }
        _ => {}
    };
}
