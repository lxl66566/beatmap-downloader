use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, HighlightSpacing, List, ListState},
    Frame,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct SelectList<'a> {
    pub list: Vec<Line<'a>>,
    pub cursor: usize,
}

impl<'a> SelectList<'a> {
    pub fn new<T>(list: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Line<'a>>,
    {
        Self {
            list: list.into_iter().map(|i| i.into()).collect(),
            cursor: 0,
        }
    }

    pub fn next(&mut self) {
        self.cursor = (self.cursor + 1).rem_euclid(self.list.len());
        assert!(self.cursor < self.list.len());
    }

    pub fn prev(&mut self) {
        self.cursor = (self.cursor as isize - 1).rem_euclid(self.list.len() as isize) as usize;
        assert!(self.cursor < self.list.len());
    }

    pub fn iter(&self) -> impl Iterator<Item = &Line> {
        self.list.iter()
    }

    /// render the select list with default style in this project.
    pub fn render(&mut self, frame: &mut Frame, block: Block, area: Rect) {
        let styled_list = List::new(self.list.clone())
            .style(Style::default().fg(Color::White))
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            )
            .block(block);
        frame.render_stateful_widget(
            styled_list,
            area,
            &mut ListState::default().with_selected(Some(self.cursor)),
        )
    }
}

#[cfg(test)]
mod tests {}
