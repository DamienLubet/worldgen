use eframe::egui;
use crate::generate::{Map, Biome};

pub struct WorldGenApp {
    map: Map,
}

impl WorldGenApp {
    pub fn new(mut map: Map) -> Self {
        Self { map }
    }
}

impl eframe::App for WorldGenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            
            // Calculer l'échelle pour adapter la carte à la fenêtre
            let available_rect = ui.available_rect_before_wrap();
            let scale_x = available_rect.width() / self.map.width;
            let scale_y = available_rect.height() / self.map.height;
            let scale = scale_x.min(scale_y) * 0.9; // 90% pour laisser une marge
            
            let offset_x = available_rect.left() + (available_rect.width() - self.map.width * scale) / 2.0;
            let offset_y = available_rect.top() + (available_rect.height() - self.map.height * scale) / 2.0;
            
            // Dessiner chaque cellule
            for cell in &self.map.cells {
                if cell.polygon.len() >= 3 {
                    // Convertir les points du polygone
                    let points: Vec<egui::Pos2> = cell.polygon.iter()
                        .map(|(x, y)| {
                            egui::Pos2::new(
                                offset_x + x * scale,
                                offset_y + y * scale
                            )
                        })
                        .collect();
                    
                    let color = match cell.biome {
                        Biome::Ocean => egui::Color32::from_rgb(50, 120, 200),  // Bleu océan
                        Biome::Forest => egui::Color32::from_rgb(34, 139, 34),  // Vert forêt
                    };
                    
                    painter.add(egui::Shape::convex_polygon(
                        points.clone(),
                        color,
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 100))
                    ));
                }
            }
            
            // Afficher des informations
            ui.label(format!("Cellules: {}", self.map.cells.len()));
            ui.label(format!("Taille: {}x{}", self.map.width, self.map.height));
            
            if ui.button("Générer la carte").clicked() {
                self.map = Map::generate(1200.0, 800.0, 1600);
                
            }
        });
    }
        
}