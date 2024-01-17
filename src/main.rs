/// TUI Application.
pub mod app;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

pub mod components;
pub mod core;
pub mod info;

use std::io;

use anyhow::Result;

use app::App;
use clap::Parser;
use core::api::{Cli, Commands};
use current_locale::current_locale;
use event::{Event, EventHandler};
use info::info;
use ratatui::{backend::CrosstermBackend, Terminal};
use rust_i18n::set_locale;
use tui::Tui;
use update::update;

#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("locales", fallback = "zh-CN");

#[tokio::main]
async fn main() -> Result<()> {
    set_locale(current_locale().unwrap_or("zh-CN".to_owned()).as_str());

    let cli = Cli::parse();
    let api = &match cli.download.as_ref().unwrap() {
        Commands::Download(api) => api.clone(),
    }
    .with_cmd("beatmaplist".to_string());
    // test
    return info(api).await;

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while !app.force_quit && app.layer > 0 {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
