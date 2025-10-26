use crate::grid::{Grid, GridError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("Grid error: {0}")]
    GridError(#[from] GridError),
}

#[derive(Debug)]
pub struct World {
    pub grid: Grid,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(width: usize, height: usize) -> Result<Self, WorldError> {
        let mut grid = Grid::new(width, height).map_err(WorldError::GridError)?;
        grid.generate();

        Ok(Self {
            grid,
            width,
            height,
        })
    }

    pub fn display(&self) {
        println!("Grid dimensions: {} x {}", self.width, self.height);
    }
}
