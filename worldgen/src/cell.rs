use crate::biome::Biome;

#[derive(Debug)]
pub struct Cell{
    pub id: usize,
    pub polygon: Vec<(f32, f32)>,
    pub position: (f32, f32),
    pub biome: Biome,
    pub temperature: f32,
    pub humidity: f32,
    pub elevation: f32,
}

impl Cell {
    pub fn new(id: usize, polygon: Vec<(f32, f32)>, position: (f32, f32), biome: Biome) -> Self {
        Self {
            id,
            polygon,
            position,
            biome,
            temperature: 0.0,
            humidity: 0.0,
            elevation: 0.0,
        }
    }

    pub fn is_inside(&self, point: (f32, f32)) -> bool {
        // TODO
        false
    }

    pub fn get_biome(&self) -> &Biome {
        &self.biome
    }
}