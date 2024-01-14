use crate::components::multi_select_list::{DefaultSelection, MultiSelectList};
use ratatui::text::Line;

/// Application.
#[derive(Debug, Default)]
pub struct App<'a> {
    /// Layer of the app. If `layer` = 0, app would quit.
    pub layer: u8,
    pub force_quit: bool,
    pub item: MultiSelectList<'a>,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        App {
            layer: 1,
            force_quit: false,
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

    /// Go back to previous layer.
    pub fn go_back(&mut self) {
        self.layer = self.layer.saturating_sub(1);
    }

    /// Force to quit the program.
    pub fn force_quit(&mut self) {
        self.force_quit = true;
    }
}
