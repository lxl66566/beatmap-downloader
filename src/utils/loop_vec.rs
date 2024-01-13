use anyhow::{ensure, Result};

pub type LoopVec<T> = Vec<T>;

/// A Vec can be loop iterated.
#[allow(unused)]
#[derive(Debug)]
pub struct LoopVecIter<T> {
    vec: LoopVec<T>,
    index: usize,
}

#[allow(unused)]
impl<T> LoopVecIter<T> {
    /// construct LoopVecIter from LoopVec
    pub fn from(vec: LoopVec<T>) -> Result<Self> {
        ensure!(!vec.is_empty(), "LoopVec cannot construct from empty vec.");
        Ok(LoopVecIter { vec, index: 0 })
    }

    /// get index of LoopVecIter
    fn index(&self) -> usize {
        self.index
    }

    /// return selected element.
    ///
    /// ```rust
    /// let v = LoopVec(!vec[1, 2, 3]);
    /// assert_eq!(v.selected(), 1);
    /// v.next();
    /// assert_eq!(v.selected(), 2);
    /// ```
    pub fn selected(&self) -> &T {
        &self.vec[self.index]
    }

    /// return mutable selected element.
    pub fn selected_mut(&mut self) -> &mut T {
        &mut self.vec[self.index]
    }

    /// set to previous element.
    ///
    /// ```rust
    /// let v = LoopVec(!vec[1, 2, 3]);
    /// assert_eq!(v.prev(), 3);
    /// ```
    pub fn prev(&mut self) -> &T {
        self.index = (self.index + self.vec.len() - 1) % self.vec.len();
        &self.vec[self.index]
    }

    /// set to next element.
    ///
    /// ```rust
    /// let v = LoopVec(!vec[1, 2, 3]);
    /// assert_eq!(v.next(), 2);
    /// ```
    pub fn next(&mut self) -> &T {
        self.index = (self.index + 1) % self.vec.len();
        &self.vec[self.index]
    }
}

pub trait IntoLoopVecIter<T> {
    fn into_loop_vec_iter(self) -> Result<LoopVecIter<T>>
    where
        Self: IntoIterator<Item = T>,
        T: Sized;
}

impl<T> IntoLoopVecIter<T> for LoopVec<T> {
    fn into_loop_vec_iter(self) -> Result<LoopVecIter<T>> {
        LoopVecIter::from(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_loop_vec_iter() {
        let mut v = vec![1, 2, 3].into_loop_vec_iter().unwrap();
        assert_eq!(v.selected(), &1);
        assert_eq!(v.index(), 0);
        assert_eq!(v.next(), &2);
        assert_eq!(v.next(), &3);
        assert_eq!(v.next(), &1);
        assert_eq!(v.prev(), &3);
        assert_eq!(v.index(), 2);
    }
}
