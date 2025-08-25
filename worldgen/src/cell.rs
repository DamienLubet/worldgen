use crate::biome::Biome;

#[derive(Debug)]
pub struct Cell{
    pub id: usize,
    pub polygon: Vec<(f32, f32)>,
    pub position: (f32, f32),
    pub biome: Biome,
    pub elevation: f64,
    pub temperature: f64,
    pub humidity: f64,
}

impl Cell {
    pub fn new(id: usize, polygon: Vec<(f32, f32)>, position: (f32, f32), biome: Biome, elevation: f64, temperature: f64, humidity: f64) -> Self {
        Self {
            id,
            polygon,
            position,
            biome,
            elevation,
            temperature,
            humidity,
        }
    }

    pub fn is_inside(&self, point: (f32, f32)) -> bool {
        // TODO
        false
    }

    pub fn get_biome(&self) -> &Biome {
        &self.biome
    }

    pub fn display(&self) {
        println!("Cell ID: {}, Position: {:?}, Biome: {:?}", self.id, self.position, self.biome);
        println!("  Elevation: {}", self.elevation);
        println!("  Temperature: {}", self.temperature);
        println!("  Humidity: {}", self.humidity);
    }
}