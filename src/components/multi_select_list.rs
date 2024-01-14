use anyhow::{Ok, Result};
use ratatui::{
    style::{Style, Stylize},
    text::Line,
};
use std::collections::BTreeSet;

/// the selected set behave in construction.
pub enum DefaultSelection {
    Empty,
    Full,
    First,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct MultiSelectList<'a> {
    pub list: Vec<Line<'a>>,
    pub cursor: usize,
    selected: BTreeSet<usize>,
    selected_color: Style,
}

impl<'a> MultiSelectList<'a> {
    pub fn new(list: Vec<Line<'a>>, default: DefaultSelection) -> Self {
        let mut temp = Self {
            list: list.clone(),
            cursor: 0,
            selected: BTreeSet::new(),
            selected_color: Style::default().green(),
        };
        match default {
            DefaultSelection::Full => {
                for i in 0..temp.list.len() {
                    temp.select(i).unwrap();
                }
            }
            DefaultSelection::First => {
                temp.select(0).unwrap();
            }
            _ => {}
        };
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
}

#[cfg(test)]
mod tests {}
