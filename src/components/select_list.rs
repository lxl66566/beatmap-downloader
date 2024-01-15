use ratatui::text::Line;

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
}

#[cfg(test)]
mod tests {}
