use rand::prelude::*;
use rand::distr::Uniform;
use voronator::{VoronoiDiagram, delaunator::Point};
use noise::{NoiseFn, Perlin};

#[derive(Debug, Clone)]
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
pub struct World{
    pub seed: u32,   // Random seed for map generation
    pub cells: Vec<Cell>,   // Cells of the world map
    pub width: f32,
    pub height: f32,
}

impl World{
    pub fn new(width: f32, height: f32, num_cells: usize) -> Self {
        let mut cells: Vec<Cell> = Vec::new();  

        // Initialize random number generator
        let mut rng = rand::rng();
        let range1 = Uniform::new(0., width).unwrap();
        let range2 = Uniform::new(0., height).unwrap();
        let mut points: Vec<(f64, f64)> = (0..num_cells)
            .map(|_| (rng.sample(&range1) as f64, rng.sample(&range2) as f64))
            .collect();

        lloyd_relaxation(&mut points, width.into(), height.into(), 2);

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
            let mut noise_value = fbm(&perlin, nx, ny, 4);

            let dx = site_point.0 - (width as f64) / 2.0;
            let dy = site_point.1 - (height as f64) / 2.0;
            let distance = ((dx * dx + dy * dy).sqrt()) / ((width as f64) / 2.0) * 0.5;
            noise_value -= distance * 2.0; // Adjust noise based on distance from center


            // Determine biome based on noise value
            let biome = if noise_value < -0.1 {
                Biome::Ocean
            } else {
                Biome::Forest
            };

            cells.push(Cell { polygon, position: (points[i].0 as f32, points[i].1 as f32), biome });
        }
        Self { seed, cells, width, height }
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

fn lloyd_relaxation(points: &mut Vec<(f64, f64)>, width: f64, height: f64, iterations: usize) {
    // Lloyd relaxation by moving points to the centroids of their Voronoi cells using the average of the cell points
    for _ in 0..iterations {
        let diagram = VoronoiDiagram::<Point>::from_tuple(&(0., 0.), &(width, height), &points).unwrap();
        let mut new_points = Vec::with_capacity(points.len());
        for (i,cell) in diagram.cells().iter().enumerate() {
            let polygon: Vec<(f64, f64)> = cell.points().iter()
                .map(|p| (p.x, p.y))
                .collect();

            if polygon.is_empty() {
                new_points.push(points[i]);
                continue;
            }
            // Calculate centroid by averaging the points
            let (mut cx, mut cy) = (0.0, 0.0);
            for &(x, y) in &polygon {
                cx += x;
                cy += y;
            }
            cx /= polygon.len() as f64;
            cy /= polygon.len() as f64;

            // Clamp to map boundaries
            cx = cx.clamp(0.0, width);
            cy = cy.clamp(0.0, height);

            new_points.push((cx, cy));
        }
        *points = new_points;
    }
}
