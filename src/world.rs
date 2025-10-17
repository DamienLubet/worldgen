use crate::grid::Grid;
use rand::prelude::*;

#[derive(Debug)]
pub struct World {
    pub grid: Grid,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = Grid::new(width, height);

        Self {
            grid,
            width,
            height,
        }
    }

    pub fn display(&self) {
        println!("Grid dimensions: {} x {}", self.width, self.height);
    }
}
