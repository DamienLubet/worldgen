use rand::prelude::*;
use rand::distr::Uniform;
use voronator::{VoronoiDiagram, delaunator::Point};
use noise::{NoiseFn, Perlin};

#[derive(Debug)]
pub enum Biome{
    Ocean,
    Forest,
}

#[derive(Debug)]
pub struct Cell{
    pub polygon: Vec<(f32, f32)>,
    pub position: (f32, f32),
    pub biome: Biome,
}

#[derive(Debug)]
pub struct Map{
    pub cells: Vec<Cell>,
    pub width: f32,
    pub height: f32,
}

impl Map{
    pub fn generate(width: f32, height: f32, num_cells: usize) -> Self {
        let mut cells: Vec<Cell> = Vec::new();  

        // Initialize random number generator
        let mut rng = rand::rng();
        let range1 = Uniform::new(0., width).unwrap();
        let range2 = Uniform::new(0., height).unwrap();
        let points: Vec<(f64, f64)> = (0..num_cells)
            .map(|_| (rng.sample(&range1) as f64, rng.sample(&range2) as f64))
            .collect();

        // Create Voronoi diagram
        let diagram = 
            VoronoiDiagram::<Point>::from_tuple(&(0., 0.), &(width as f64, height as f64), &points).unwrap();

        // Perlin noise
        let seed: u32 = rng.random();
        let perlin = Perlin::new(seed);
        let noise_scale = 0.0025;

        for (i, cell) in diagram.cells().into_iter().enumerate(){
            let polygon: Vec<(f32, f32)> = cell.points().iter().map(|p| (p.x as f32, p.y as f32)).collect();

            let site_point = points[i];
            let nx = site_point.0 * noise_scale;
            let ny = site_point.1 * noise_scale;
            let mut noise_value = fbm(&perlin, nx, ny, 8);

            let dx = site_point.0 - (width as f64) / 2.0;
            let dy = site_point.1 - (height as f64) / 2.0;
            let distance = ((dx * dx + dy * dy).sqrt()) / ((width as f64) / 2.0) * 0.5;
            noise_value -= distance; 


            // Determine biome based on noise value
            let biome = if noise_value < -0.1 {
                Biome::Ocean
            } else {
                Biome::Forest
            };


            cells.push(Cell { polygon, position: (nx as f32, ny as f32), biome });

        }

        Self { cells, width, height }
    }


    pub fn display(&self) {
        println!("{} cellules générées", self.cells.len());
        for i in 0..self.cells.len() {
            println!("{:?}", self.cells[i]);
        }
    }

    pub fn display_biomes(&self) {
        for cell in &self.cells {
            println!("Cell at position {:?} has biome {:?}", cell.position, cell.biome);
        }
    }
}


// Fractal Brownian Motion (FBM) for Perlin noise
fn fbm(perlin: &Perlin, x: f64, y: f64, octaves: usize) -> f64 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;

    for _ in 0..octaves {
        value += amplitude * perlin.get([x * frequency, y * frequency]);
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    value
}

fn calculate_polygon_centroid(polygon: &[(f32, f32)]) -> (f32, f32) {
     let mut area = 0.0;
    let mut cx = 0.0;
    let mut cy = 0.0;

    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();
        let (x0, y0) = polygon[i];
        let (x1, y1) = polygon[j];

        let a = x0 * y1 - x1 * y0;
        area += a;
        cx += (x0 + x1) * a;
        cy += (y0 + y1) * a;
    }
    area *= 0.5;

    if area.abs() < 1e-10{
        let sum_x = polygon.iter().map(|&(x, _)| x).sum::<f32>();
        let sum_y = polygon.iter().map(|&(_, y)| y).sum::<f32>();
        return (sum_x / polygon.len() as f32, sum_y / polygon.len() as f32)
    } 
    cx /= 6.0 * area;
    cy /= 6.0 * area;
    (cx, cy)
}
