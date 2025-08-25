use eframe::egui;
use crate::world::{World};
use crate::biome::Biome;

pub struct WorldGenApp {
    map: World,
    temperature: bool,
}

impl WorldGenApp {
    pub fn new(map: World) -> Self {
        Self { map, temperature: false }
    }
}

impl eframe::App for WorldGenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            
            let available_rect = ui.available_rect_before_wrap();
            let scale_x = available_rect.width() / self.map.width;
            let scale_y = available_rect.height() / self.map.height;
            let scale = scale_x.min(scale_y) * 0.9; 
            
            let offset_x = available_rect.left() + (available_rect.width() - self.map.width * scale) / 2.0;
            let offset_y = available_rect.top() + (available_rect.height() - self.map.height * scale) / 2.0;
            
            for cell in &self.map.cells {
                if cell.polygon.len() >= 3 {
                    
                    let points: Vec<egui::Pos2> = cell.polygon.iter()
                        .map(|(x, y)| {
                            egui::Pos2::new(
                                offset_x + x * scale,
                                offset_y + y * scale
                            )
                        })
                        .collect();
                    
                    let color = match cell.biome {
                        Biome::Ocean => egui::Color32::from_rgb(50, 120, 200),  // Blue
                        Biome::Forest => egui::Color32::from_rgb(34, 139, 34),  // Green
                        Biome::Mountain => egui::Color32::from_rgb(87, 29, 29),  // Brown
                    };
                    
                    painter.add(egui::Shape::convex_polygon(
                        points.clone(),
                        color,
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 100))
                    ));
                
                    if self.temperature{
                        let color = match cell.temperature {
                            t if t < 0.25 => egui::Color32::from_rgb(0, 0, 255),     // Blue for cold
                            t if t < 0.5 => egui::Color32::from_rgb(0, 255, 255),   // Cyan for cool
                            t if t < 0.75 => egui::Color32::from_rgb(255, 255, 0),  // Yellow for moderate
                            t if t < 1.0 => egui::Color32::from_rgb(255, 0, 0),   // Red for hot
                            _ => egui::Color32::from_rgb(128, 128, 128),
                        };

                        let transparent_color = egui::Color32::from_rgba_unmultiplied(
                            color.r(), color.g(), color.b(), 80
                        );

                        painter.add(egui::Shape::convex_polygon(
                            points.clone(),
                            transparent_color,
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 100))
                        ));
                    }
                }
            }
            
            ui.label(format!("Cell: {}", self.map.cells.len()));
            ui.label(format!("Size: {}x{}", self.map.width, self.map.height));
            
            if ui.button("New Map").clicked() {
                self.map = World::new(1920.0, 1080.0, 16000);
            }
            if ui.button("Toggle Temperature").clicked() {
                self.temperature = !self.temperature;
            }
        });
    }   
}