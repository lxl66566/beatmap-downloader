use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use time::Duration;

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    if key_event.modifiers == KeyModifiers::CONTROL
        && (key_event.code == KeyCode::Char('c') || key_event.code == KeyCode::Char('C'))
    {
        return app.force_quit();
    }
    match key_event.code {
        KeyCode::Esc => app.go_back(),
        KeyCode::Left => app.prev_session(),
        KeyCode::Right => app.next_session(),
        _ => update_app_session(app, key_event),
    };
}

pub fn update_app_session(app: &mut App, key_event: KeyEvent) {
    let key = key_event.code;
    match app.session {
        0 => match key {
            KeyCode::Up => app.mode.prev(),
            KeyCode::Down => app.mode.next(),
            _ => app.help(),
        },
        1 => match app.mode.cursor {
            0 => update_calendar(app, key_event),
            1 | 2 => {
                app.text.input(key_event);
            }
            3 => {
                app.text2.input(key_event);
            }
            _ => unreachable!("mode numeber exceeded."),
        },
        2 | 3 => {
            let multilist = if app.session == 2 {
                &mut app.item
            } else {
                &mut app.map_state
            };
            match key {
                KeyCode::Down => multilist.next(),
                KeyCode::Up => multilist.prev(),
                KeyCode::Enter => {
                    multilist.select_cursor();
                }
                _ => app.help(),
            };
        }
        _ => unreachable!("session numeber exceeded."),
    }
}

pub fn update_calendar(app: &mut App, key_event: KeyEvent) {
    let day = if key_event.modifiers == KeyModifiers::SHIFT {
        30
    } else {
        1
    };
    match key_event.code {
        KeyCode::Up => app.date = app.date.saturating_sub(Duration::days(day)),
        KeyCode::Down => app.date = app.date.saturating_add(Duration::days(day)),
        _ => {} // do nothing
    }
}
