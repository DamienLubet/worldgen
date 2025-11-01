use crate::noise::NoiseGenerator;
use rand::Rng;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GridError {
    #[error("Invalid grid dimensions")]
    InvalidDimensions,
}

#[derive(Debug)]
pub struct Grid {
    seed: u32,
    width: usize,
    height: usize,
    noise: NoiseGenerator,
    height_map: Vec<f32>,
    temperature_map: Vec<f32>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Result<Self, GridError> {
        if width <= 0 || height <= 0 {
            return Err(GridError::InvalidDimensions);
        }

        let seed: u32 = rand::rng().random();

        let size = width * height;
        let height_map = vec![0.0; size];
        let temperature_map = vec![0.0; size];

        let noise = NoiseGenerator::new(seed);

        Ok(Self {
            seed,
            width,
            height,
            noise,
            height_map,
            temperature_map,
        })
    }

    pub fn generate(&mut self) {
        let width_f = self.width as f32;
        let height_f = self.height as f32;

        // Generate height map in parallel
        self.height_map
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, value)| {
                let x = (index % self.width) as f32;
                let y = (index / self.width) as f32;
                *value = elevation(x, y, width_f, height_f, &self.noise);
            });

        // Generate temperature map in parallel
        self.temperature_map
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, value)| {
                let x = (index % self.width) as f32;
                let y = (index / self.width) as f32;
                let altitude = self.height_map[index];
                *value = temperature(x, y, height_f, altitude, &self.noise);
            });
    }

    pub fn get_height_at(&self, x: usize, y: usize) -> f32 {
        self.height_map[y * self.width + x]
    }

    pub fn get_temperature_at(&self, x: usize, y: usize) -> f32 {
        self.temperature_map[y * self.width + x]
    }
    
    pub fn get_seed(&self) -> u32 {
        self.seed
    }

    pub fn neighbors_is_sea(&self, x: usize, y: usize) -> bool {
        // Check if there is at least one neighbor below sea level
        if self.get_height_at(x, y) < 0.30 { return false; } // Skip if current is sea
        let x_min = if x > 0 { x - 1 } else { x };
        let y_min = if y > 0 { y - 1 } else { y };
        let x_max = (x + 1).min(self.width - 1);
        let y_max = (y + 1).min(self.height - 1);
        
        for neighbors_x in x_min..=x_max {
            for neighbors_y in y_min..=y_max{
                if neighbors_x == x || neighbors_y == y { continue; }
                if self.get_height_at(neighbors_x, neighbors_y) < 0.30 { return true; } 
            }
        }
        false
    }
}

fn elevation(x: f32, y: f32, width: f32, height: f32, noise: &NoiseGenerator) -> f32 {
    // Elevation calculation combining continent and detail noise with distance falloff
    const SEA_LEVEL: f32 = 0.30; 
    const MARGIN_X: f32 = 0.8;
    const MARGIN_Y: f32 = 1.0;
    const FALLOFF: f32 = 0.45;
    const DETAIL_FACTOR: f32 = 0.425;
    
    let continent = noise.continent_map(x, y);

    let distance = distance(x, y, width, height, MARGIN_X, MARGIN_Y);
    let mut elevation = continent - distance * FALLOFF;
    if SEA_LEVEL < elevation {
        let detail = noise.detail_map(x, y);
        elevation += detail * DETAIL_FACTOR;
    }
    elevation.clamp(0.0, 1.0)
}

fn temperature(x: f32, y: f32, height: f32, altitude: f32, noise: &NoiseGenerator) -> f32 {
    // Temperature calculation based on latitude, altitude, and noise
    let latitude = y / height;
    let lat_temp = 1.0 - (latitude - 0.5).abs() * 2.0; // 1.0 at equator, 0.0 at poles
    let alt_temp = 1.0 - altitude.powf(4.0); // Decrease temperature with altitude
    let noise_temp = noise.temperature_noise(x, y);
    (0.6 * lat_temp + 0.2 * alt_temp + 0.2 * noise_temp).clamp(0.0, 1.0)
}

fn distance(x: f32, y: f32, width: f32, height: f32, margin_x: f32, margin_y: f32) -> f32 {
    let cx = (width - 1.0) * 0.5;
    let cy = (height - 1.0) * 0.5;
    let rx = cx / margin_x;
    let ry = cy / margin_y;
    let nx = ((x - cx) / rx).abs(); 
    let ny = ((y - cy) / ry).abs();
    ((nx * nx + ny * ny).sqrt()).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(10, 10).unwrap();
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.height_map.len(), 100);
        assert_eq!(grid.temperature_map.len(), 100);
    }
    #[test]
    fn test_invalid_grid_creation() {
        let result = Grid::new(0, 10);
        assert!(result.is_err());
    }
    #[test]
    fn test_height_and_temperature_generation() {
        let mut grid = Grid::new(10, 10).unwrap();
        grid.generate();
        for y in 0..10 {
            for x in 0..10 {
                let height = grid.get_height_at(x, y);
                let temperature = grid.get_temperature_at(x, y);
                assert!(height >= 0.0 && height <= 1.0);
                assert!(temperature >= 0.0 && temperature <= 1.0);
            }
        }
    }
    #[test]
    fn test_get_seed() {
        let grid = Grid::new(10, 10).unwrap();
        let seed = grid.get_seed();
        assert!(seed == grid.seed);
    }
}