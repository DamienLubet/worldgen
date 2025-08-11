
mod world;
mod ui;

use world::World;
use ui::WorldGenApp;

fn main() {
    let map = World::new(1920.0, 1080.0, 5000);
    let app = WorldGenApp::new(map);
    let _ = eframe::run_native(
        "World Generation Biomes",
        eframe::NativeOptions::default(),
        Box::new(move |cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(app))
        }),
    );
}