use crate::{
    components::multi_select_list::{DefaultSelection, MultiSelectList},
    components::select_list::SelectList,
};
use ratatui::text::Line;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    Help,
    #[default]
    Main,
}

const SESSION_MAX: usize = 4;

/// Application.
#[derive(Debug, Default)]
pub struct App<'a> {
    /// Layer of the app. If `layer` = 0, app would quit.
    pub layer: u8,
    pub force_quit: bool,
    /// beatmap type: after a day / hotest / newest / search
    pub mode: SelectList<'a>,
    pub item: MultiSelectList<'a>,
    pub page: Page,
    /// Paragraph of the current page.
    pub session: usize,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        App {
            layer: 1,
            force_quit: false,
            mode: SelectList::new(
                [
                    t!("mode.date"),
                    t!("mode.hot"),
                    t!("mode.new"),
                    t!("mode.search"),
                ]
                .into_iter()
                .map(Line::raw),
            ),
            item: MultiSelectList::new(
                [
                    t!("gamemode.std"),
                    t!("gamemode.taiko"),
                    t!("gamemode.ctb"),
                    t!("gamemode.mania"),
                ]
                .into_iter()
                .map(Line::raw),
                DefaultSelection::Full,
            ),
            page: Page::default(),
            session: 0,
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

    /// Return current page.
    pub fn current_page(&self) -> Page {
        if self.layer == 1 {
            Page::Main
        } else {
            Page::Help
        }
    }

    /// Display help page.
    pub fn help(&mut self) {
        if self.page == Page::Help {
            self.layer -= 1;
            self.page = Page::Main;
        } else {
            self.layer += 1;
            self.page = Page::Help;
        }
    }

    pub fn next_session(&mut self) {
        self.session = (self.session + 1) % SESSION_MAX;
    }

    pub fn prev_session(&mut self) {
        self.session = (self.session as isize - 1).rem_euclid(SESSION_MAX as isize) as usize;
    }
}
