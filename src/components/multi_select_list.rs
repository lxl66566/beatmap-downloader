use anyhow::{Ok, Result};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, HighlightSpacing, List, ListState},
    Frame,
};
use std::collections::BTreeSet;

/// the selected set behave in construction.
pub enum DefaultSelection {
    Empty,
    Full,
    First,
    Partial(Vec<usize>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct MultiSelectList<'a> {
    pub list: Vec<Line<'a>>,
    pub cursor: usize,
    selected: BTreeSet<usize>,
    selected_color: Style,
}

impl<'a> MultiSelectList<'a> {
    pub fn new<T>(list: T, default: DefaultSelection) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Line<'a>>,
    {
        let mut temp = Self {
            list: list.into_iter().map(|i| i.into()).collect(),
            cursor: 0,
            selected: Default::default(),

            selected_color: Style::default().green(),
        };
        temp.selected = match default {
            DefaultSelection::Full => (0..temp.list.len()).collect(),
            DefaultSelection::First => BTreeSet::from([0]),
            DefaultSelection::Empty => BTreeSet::new(),
            DefaultSelection::Partial(set) => set.into_iter().collect(),
        };
        // we need to patch style to all lines, including white line.
        for (i, line) in temp.list.iter_mut().enumerate() {
            line.patch_style(if temp.selected.contains(&i) {
                temp.selected_color
            } else {
                Style::default().white()
            })
        }
        temp
    }

    /// Select the given index. If index has been selected, it will cancel the selection.
    pub fn select(&mut self, index: usize) -> Result<&Line> {
        let line = self.list.get_mut(index).expect("select index out of range");
        if self.selected.insert(index) {
            line.patch_style(self.selected_color);
        } else {
            self.selected.remove(&index);
            line.patch_style(Style::default().white())
        }
        Ok(line)
    }

    /// Select the cursor index.
    pub fn select_cursor(&mut self) -> &Line {
        self.select(self.cursor).unwrap()
    }

    // Set the color of selected items. `color` would be patched to Line.
    pub fn set_selected_color(mut self, color: Style) -> Self {
        self.selected_color = color;
        self
    }

    pub fn next(&mut self) {
        self.cursor = (self.cursor + 1).rem_euclid(self.list.len());
        assert!(self.cursor < self.list.len());
    }

    pub fn prev(&mut self) {
        self.cursor = (self.cursor as isize - 1).rem_euclid(self.list.len() as isize) as usize;
        assert!(self.cursor < self.list.len());
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn iter(&self) -> impl Iterator<Item = &Line> {
        self.list.iter()
    }

    /// get the bit_sum of the MultiSelectList.
    pub fn bit_sum(&self) -> u32 {
        self.selected.iter().fold(0, |acc, x| acc | (1 << x))
    }

    /// render the select list with default style in this project.
    pub fn render(&mut self, frame: &mut Frame, block: Block, area: Rect) {
        let styled_list = List::new(self.list.clone())
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol("> ")
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .block(block);
        frame.render_stateful_widget(
            styled_list,
            area,
            &mut ListState::default().with_selected(Some(self.cursor)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_select() {
        let mut list = MultiSelectList::new(
            vec!["Alice", "Bob", "Carol", "David"]
                .into_iter()
                .map(Line::from),
            DefaultSelection::First,
        );
        assert_eq!(list.cursor(), 0);
        assert_eq!(list.selected, BTreeSet::from([0]));
        list.next();
        assert_eq!(list.cursor(), 1);
        list.select(2).unwrap();
        assert_eq!(list.selected, BTreeSet::from([0, 2]));
        assert_eq!(list.bit_sum(), 0b101);
    }
}
