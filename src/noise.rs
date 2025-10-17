use noise::{Billow, Fbm, NoiseFn, Perlin, RidgedMulti, ScalePoint, Seedable, Turbulence};

// Noise generation using Perlin noise

#[derive(Debug)]
pub struct NoiseGenerator {
    pub seed: u32,
    pub perlin: Perlin,
    pub fbm: Fbm<Perlin>,
    pub scale: f64,
}

impl NoiseGenerator {
    pub fn new(seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        let scale: f64 = 0.003;

        let mut fbm = Fbm::<Perlin>::new(seed);
        fbm.frequency = 1.0;
        fbm.lacunarity = 2.0;
        fbm.persistence = 0.5;
        fbm.octaves = 4;

        Self {
            seed,
            perlin,
            fbm,
            scale,
        }
    }

    // Basic height map generation using FBM
    pub fn height_map(&self, x: f32, y: f32) -> f32 {
        let height_noise = ScalePoint::new(self.fbm.clone()).set_scale(self.scale);
        ((height_noise.get([x as f64, y as f64]) + 1.0) * 0.5) as f32
    }

    // Temperature noise for biome generation
    pub fn temperature_noise(&self, x: f32, y: f32) -> f32 {
        let temp_noise =
            ScalePoint::new(self.perlin.clone().set_seed(self.seed + 1)).set_scale(self.scale);
        ((temp_noise.get([x as f64, y as f64]) + 1.0) * 0.5) as f32
    }
}
