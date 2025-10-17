use worldgen::ui::WorldGenApp;
use worldgen::world::World;

fn main() {
    let world: World = World::new(1920, 1000);
    let app = WorldGenApp::new(world);
    let _ = eframe::run_native(
        "World Generation Biomes",
        eframe::NativeOptions::default(),
        Box::new(move |cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(app))
        }),
    );
}
