use noise::{Fbm, NoiseFn, Perlin, ScalePoint, Seedable, Simplex};

// Noise generation using Perlin noise

#[derive(Debug)]
pub struct NoiseGenerator {
    pub seed: u32,
    pub perlin: Perlin,
    pub continent_scale: f64,
    pub detail_scale: f64,
    pub continent_fbm: Fbm<Perlin>,
    pub detail_fbm: Fbm<Simplex>,
}

impl NoiseGenerator {
    pub fn new(seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        
        let continent_scale: f64 = 0.004;
        let mut continent_fbm: Fbm<Perlin> = Fbm::new(seed);
        continent_fbm.frequency = 1.0;
        continent_fbm.lacunarity = 2.1;
        continent_fbm.persistence = 0.55;
        continent_fbm.octaves = 6;
        
        let detail_scale: f64 = 0.015;
        let mut detail_fbm: Fbm<Simplex> = Fbm::new(seed + 1);
        detail_fbm.frequency = 1.0;
        detail_fbm.lacunarity = 2.0;
        detail_fbm.persistence = 0.5;
        detail_fbm.octaves = 5;

        Self {
            seed,
            perlin,
            continent_scale,
            detail_scale,
            continent_fbm,
            detail_fbm,
        }
    }

    // Basic height map generation using FBM
    pub fn continent_map(&self, x: f32, y: f32) -> f32 {
        let scale_x = x as f64 * self.continent_scale;
        let scale_y = y as f64 * self.continent_scale;
        (self.continent_fbm.get([scale_x, scale_y]) * 0.5 + 0.5) as f32
    }

    // Detailed height variations using another FBM
    pub fn detail_map(&self, x: f32, y: f32) -> f32 {
        let scale_x = x as f64 * self.detail_scale;
        let scale_y = y as f64 * self.detail_scale;
        (self.detail_fbm.get([scale_x, scale_y]) * 0.5 + 0.5) as f32
    }

    // Temperature noise for biome generation
    pub fn temperature_noise(&self, x: f32, y: f32) -> f32 {
        let scale_x = x as f64 * self.continent_scale;
        let scale_y = y as f64 * self.continent_scale;
        (self.perlin.get([scale_x, scale_y]) * 0.5 + 0.5) as f32
    }
}
