use crate::noise::NoiseGenerator;
use rand::Rng;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Grid {
    seed: u32,  
    width: usize,
    height: usize,
    noise: NoiseGenerator,
    pub height_map: Vec<f32>,
    pub temperature_map: Vec<f32>,
}

impl Grid{
    pub fn new(width : usize, height: usize) -> Self {
        let seed: u32 = rand::rng().random();

        let size = width * height;
        let mut height_map = vec![0.0; size];
        let mut temperature_map = vec![0.0; size];

        let noise = NoiseGenerator::new(seed);

        // Generate height map in parallel
        height_map.par_iter_mut().enumerate().for_each(|(index, value)| {
            let x = index % width;
            let y = index / width;
            *value = elevation(x as f32, y as f32, width as f32, height as f32, &noise)
        });

        // Generate temperature map in parallel
        temperature_map.par_iter_mut().enumerate().for_each(|(index, value)| {
            let x = index % width;
            let y = index / width;
            *value = temperature(x as f32, y as f32, height as f32, height_map[index], &noise);
        });

        Self { seed, width, height, noise, height_map, temperature_map }
    }
}


fn elevation(x: f32, y: f32, width: f32, height: f32, noise: &NoiseGenerator) -> f32 {
    // Elevation calculation based on the center of the map and noise
    let elevation = noise.height_map(x, y);
    let dx = (x - (width / 2.0)) / 2.0;
    let dy = y - (height / 2.0);
    let distance = ((dx * dx + dy * dy).sqrt()) / (width / 2.0) * 0.5;
    (elevation - distance * 1.50).clamp(0.0, 1.0)
}

fn temperature(x: f32, y: f32, height: f32, altitude: f32, noise: &NoiseGenerator) -> f32 {
    // Temperature calculation based on latitude, altitude, and noise
    let latitude = y / height;
    let lat_temp = 1.0 - (latitude - 0.5).abs() * 2.0; // 1.0 at equator, 0.0 at poles
    let alt_temp = 1.0 - altitude.powf(4.0); // Decrease temperature with altitude
    let noise_temp = noise.temperature_noise(x, y);
    (0.6 * lat_temp + 0.2 * alt_temp + 0.2 * noise_temp).clamp(0.0, 1.0)
}