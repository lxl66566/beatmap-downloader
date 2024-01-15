/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

pub mod components;
mod util;

use anyhow::Result;

use app::App;
use current_locale::current_locale;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use rust_i18n::set_locale;
use tui::Tui;
use update::update;

#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("locales", fallback = "zh-CN");
fn main() -> Result<()> {
    set_locale(current_locale().unwrap_or("zh-CN".to_owned()).as_str());

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.force_quit && app.layer > 0 {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
