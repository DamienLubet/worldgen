// Biome types for the world generation

#[derive(Debug, Clone)]
pub enum Biome{
    Ocean,
    Forest,
    Mountain,
    //etc. 
}

pub fn determine_biome(elevation: f32, temperature: f32, humidity: f32) -> Biome {
    // TODO
    Biome::Forest
}