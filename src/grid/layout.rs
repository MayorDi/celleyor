use std::ops::Index;

use super::constants::SIZE_GRID;

pub struct Layout<T> {
    pub is_need_render: bool,
    count_zones: usize,
    inner: Vec<Option<T>>,
}

impl<T: Clone> Layout<T> {
    pub fn new() -> Self {
        Self {
            is_need_render: false,
            count_zones: 0,
            inner: [const { None }; SIZE_GRID[0] * SIZE_GRID[1]].to_vec(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn set_by_idx(&mut self, index: usize, value: T) {
        self.inner[index] = Some(value);
        self.count_zones += 1;
        self.is_need_render = self.count_zones != 0;
    }

    pub fn set_by_pos(&mut self, pos: nalgebra::Vector2<usize>, value: T) {
        self.inner[pos_to_idx(pos)] = Some(value);
        self.count_zones += 1;
        self.is_need_render = self.count_zones != 0;
    }

    pub fn del_by_idx(&mut self, index: usize) {
        self.inner[index] = None;
        self.count_zones -= 1;
        self.is_need_render = self.count_zones != 0;
    }

    pub fn del_by_pos(&mut self, pos: nalgebra::Vector2<usize>) {
        self.inner[pos_to_idx(pos)] = None;
        self.count_zones -= 1;
        self.is_need_render = self.count_zones != 0;
    }
}

impl<T> Index<nalgebra::Vector2<usize>> for Layout<T> {
    type Output = Option<T>;
    fn index(&self, index: nalgebra::Vector2<usize>) -> &Self::Output {
        &self.inner[index.x]
    }
}

impl<T> Index<usize> for Layout<T> {
    type Output = Option<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

#[inline]
pub fn pos_to_idx(pos: nalgebra::Vector2<usize>) -> usize {
    pos.y * SIZE_GRID[0] + pos.x
}

#[inline]
pub const fn idx_to_pos(index: usize) -> nalgebra::Vector2<usize> {
    nalgebra::Vector2::new((index % SIZE_GRID[0]), (index / SIZE_GRID[0]))
}
