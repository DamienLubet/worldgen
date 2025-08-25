use rand::prelude::*;
use rand::distr::Uniform;
use voronator::{VoronoiDiagram, delaunator::Point};
use noise::{NoiseFn, Perlin};
use crate::noise::*;
use crate::biome::Biome;
use crate::cell::Cell;

#[derive(Debug)]
pub struct World{
    pub seed: u32,   // Random seed for map generation
    pub num_cells: usize,
    pub cells: Vec<Cell>,   // Cells of the world map
    pub width: f32,
    pub height: f32,

}

impl World{
    pub fn new(width: f32, height: f32, num_cells: usize) -> Self {
        let points = generate_points(width, height, num_cells);
        let diagram = VoronoiDiagram::<Point>::from_tuple(&(0., 0.), &(width as f64, height as f64), &points).unwrap();
        let seed: u32 = rand::rng().random();
        let cells = create_cells(diagram, points, width, height, seed);

        Self { seed, num_cells, cells, width, height }
    }

    pub fn display(&self) {
        println!("{} cells generated", self.num_cells);
        for i in 0..self.cells.len() {
            self.cells[i].display();
        }
    }

    pub fn display_biomes(&self) {
        for cell in &self.cells {
            println!("Cell at position {:?} has biome {:?}", cell.position, cell.biome);
        }
    }

    pub fn get_cell(&self, position: (f32, f32)) -> Option<&Cell> {
        // Find the cell that contains the given position
        self.cells.iter().find(|cell| cell.is_inside(position))
    }
}

fn generate_points(width: f32, height: f32, num_cells: usize) -> Vec<(f64, f64)> {
    // Generate random points
    let mut rng = rand::rng();
    let range1 = Uniform::new(0., width).unwrap();
    let range2 = Uniform::new(0., height).unwrap();
    let mut points: Vec<(f64, f64)> = (0..num_cells)
        .map(|_| (rng.sample(&range1) as f64, rng.sample(&range2) as f64))
        .collect();

    lloyd_relaxation(&mut points, width.into(), height.into(), 2);
    points
}

fn create_cells(diagram: VoronoiDiagram<Point>, points: Vec<(f64, f64)>, width: f32, height: f32, seed: u32) -> Vec<Cell> {
    // Create cells using the voronoi diagram
    let mut cells: Vec<Cell> = Vec::new();
    
    // Perlin noise
    let noise = NoiseGenerator::new(seed);

    for (i, cell) in diagram.cells().into_iter().enumerate(){
        let polygon: Vec<(f32, f32)> = cell.points().iter().map(|p| (p.x as f32, p.y as f32)).collect();

        // Calculate elevation
        let elevation = elevation(points[i].0, points[i].1, width.into(), height.into(), &noise);

        // Calculate temperature
        let temperature = temperature(points[i].0, points[i].1, height.into(), elevation, &noise);

        // Calculate humidity
        let humidity = 0.0;  // TODO

        // Determine biome based on noise value
        let biome = if elevation < 0.30 {
            Biome::Ocean
        } else if elevation > 0.6 {
            Biome::Mountain
        } else {
            Biome::Forest
        };

        cells.push(Cell::new(
            i,
            polygon,
            (points[i].0 as f32, points[i].1 as f32),
            biome,
            elevation,
            temperature,
            humidity,
        ));
    }
    
    cells
}

fn elevation(x: f64, y: f64, width: f64, height: f64, noise: &NoiseGenerator) -> f64 {
    // Elevation calculation based on the center of the map and noise
    let elevation = noise.height_map(x, y);
    let dx = (x - (width as f64) / 2.0) / 2.0;
    let dy = y - (height as f64) / 2.0;
    let distance = ((dx * dx + dy * dy).sqrt()) / ((width as f64) / 2.0) * 0.5;
    (elevation - distance * 1.50).clamp(0.0, 1.0)
}

fn temperature(x: f64, y: f64, height: f64, altitude: f64, noise: &NoiseGenerator) -> f64 {
    // Temperature calculation based on latitude, altitude, and noise
    let latitude = y / height;
    let lat_temp = 1.0 - (latitude - 0.5).abs() * 2.0; // 1.0 at equator, 0.0 at poles
    let alt_temp = 1.0 - altitude.powf(4.0); // Decrease temperature with altitude
    let noise_temp = noise.temperature_noise(x, y);
    (0.6 * lat_temp + 0.2 * alt_temp + 0.2 * noise_temp).clamp(0.0, 1.0)
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
