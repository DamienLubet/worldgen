use rand::prelude::*;
use rand::distr::Uniform;
use voronator::{VoronoiDiagram, delaunator::Point};

fn main() {
    let mut rng = rand::rng();
    let range1 = Uniform::new(0., 100.).unwrap();
    let range2 = Uniform::new(0., 100.).unwrap();
    let points: Vec<(f64, f64)> = (0..100)
        .map(|_| (rng.sample(&range1), rng.sample(&range2)))
        .collect();

    let diagram = VoronoiDiagram::<Point>::from_tuple(&(0., 0.), &(100., 100.), &points).unwrap();
     
    for cell in diagram.cells() {
        let p: Vec<(f32, f32)> = cell.points().into_iter()
            .map(|x| (x.x as f32, x.y as f32))
            .collect();
         
        println!("{:?}", p);
    }
}