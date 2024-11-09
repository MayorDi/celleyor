use constants::SIZE_GRID;

use crate::cell::Cell;

pub mod constants;

pub struct Grid {
    layout_cells: [[Option<Cell>; SIZE_GRID[0]]; SIZE_GRID[1]],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            layout_cells: [const { [const { None }; SIZE_GRID[0]] }; SIZE_GRID[1]],
        }
    }
}
