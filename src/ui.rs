use crate::world::World;
use eframe::egui;

pub struct WorldGenApp {
    map: World,
    temperature: bool,
    texture: Option<egui::TextureHandle>,
    dirty: bool,
}

impl WorldGenApp {
    pub fn new(map: World) -> Self {
        Self {
            map,
            temperature: false,
            texture: None,
            dirty: true,
        }
    }

    fn rebuild_texture(&mut self, ctx: &egui::Context) {
        let width = self.map.width;
        let height = self.map.height;
        if width == 0 || height == 0 {
            return;
        }

        let mut pixels = vec![egui::Color32::BLACK; width * height];

        let water_level: f32 = 0.30;
        let mountain_level: f32 = 0.60;

        for i in 0..(width) {
            for j in 0..(height) {
                let h = self.map.grid.get_height_at(i, j);

                let base = if h < water_level {
                    egui::Color32::from_rgb(50, 120, 200)
                } else if h > mountain_level {
                    egui::Color32::from_rgb(87, 29, 29)
                } else {
                    egui::Color32::from_rgb(34, 139, 34)
                };

                let final_color = if self.temperature {
                    let t = self.map.grid.get_temperature_at(i, j);
                    let overlay = if t < 0.25 {
                        egui::Color32::from_rgb(0, 0, 255)
                    } else if t < 0.5 {
                        egui::Color32::from_rgb(0, 255, 255)
                    } else if t < 0.75 {
                        egui::Color32::from_rgb(255, 255, 0)
                    } else if t < 1.0 {
                        egui::Color32::from_rgb(255, 0, 0)
                    } else {
                        egui::Color32::from_rgb(128, 128, 128)
                    };
                    egui::Color32::from_rgb(
                        ((base.r() as u16 + overlay.r() as u16) / 2) as u8,
                        ((base.g() as u16 + overlay.g() as u16) / 2) as u8,
                        ((base.b() as u16 + overlay.b() as u16) / 2) as u8,
                    )
                } else {
                    base
                };

            pixels[j * width + i] = final_color;
        }}

        let image = egui::ColorImage {
            size: [width, height],
            source_size: egui::vec2(width as f32, height as f32),
            pixels,
        };
        let tex = ctx.load_texture("world_map", image, egui::TextureOptions::NEAREST);
        self.texture = Some(tex);
        self.dirty = false;
    }
}

impl eframe::App for WorldGenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.dirty {
            self.rebuild_texture(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Zone scrollable pour l'image
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    if let Some(tex) = &self.texture {
                        let size = egui::vec2(self.map.width as f32, self.map.height as f32);
                        ui.image((tex.id(), size));
                    } else {
                        ui.label("Aucune texture.");
                    }
                });
        });

        // Barre de contrôle fixe en haut
        egui::TopBottomPanel::top("top_controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Size: {}x{}", self.map.width, self.map.height));

                if ui.button("New Map").clicked() {
                    match World::new(self.map.width, self.map.height) {
                        Ok(new_map) => {
                            self.map = new_map;
                        }
                        Err(e) => {
                            eprintln!("Error generating new map: {:?}", e);
                        }
                    }
                    self.dirty = true;
                }
                if ui.button("Toggle Temperature").clicked() {
                    self.temperature = !self.temperature;
                    self.dirty = true;
                }
            });
        });
    }
}
