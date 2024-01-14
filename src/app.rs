use crate::components::multi_select_list::{DefaultSelection, MultiSelectList};
use ratatui::text::Line;

/// Application.
#[derive(Debug, Default)]
pub struct App<'a> {
    pub should_quit: bool,
    pub item: MultiSelectList<'a>,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        App {
            should_quit: false,
            item: MultiSelectList::new(
                [
                    Line::from(vec!["Fork tui-rs 💻".into()]),
                    Line::from(vec!["Create a website & book 🕮".into()]),
                    Line::from(vec!["Celebrate 500th commit ⭐".into()]),
                    Line::from(vec!["Celebrate 1000th commit ✨".into()]),
                    Line::from(vec!["Release Ratatui 1.0.0 🎉".into()]),
                ]
                .into(),
                DefaultSelection::Full,
            ),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
