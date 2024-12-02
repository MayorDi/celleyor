use std::ops::{Index, IndexMut};

use super::constants::SIZE_GRID;

pub struct Layout<T> {
    inner: [[Option<T>; SIZE_GRID[0]]; SIZE_GRID[1]],
}

impl<T> Layout<T> {
    pub fn new() -> Self {
        Self {
            inner: [const { [const { None }; SIZE_GRID[0]] }; SIZE_GRID[1]],
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, [Option<T>; SIZE_GRID[0]]> {
        self.inner.iter()
    }
}

impl<T> Index<nalgebra::Vector2<usize>> for Layout<T> {
    type Output = Option<T>;
    fn index(&self, index: nalgebra::Vector2<usize>) -> &Self::Output {
        &self.inner[index.x][index.y]
    }
}

impl<T> IndexMut<nalgebra::Vector2<usize>> for Layout<T> {
    fn index_mut(&mut self, index: nalgebra::Vector2<usize>) -> &mut Self::Output {
        &mut self.inner[index.x][index.y]
    }
}

impl<T> Index<usize> for Layout<T> {
    type Output = [Option<T>];
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T> IndexMut<usize> for Layout<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}
