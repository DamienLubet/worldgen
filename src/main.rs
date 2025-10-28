use worldgen::ui::WorldGenApp;
use worldgen::world::World;
use anyhow::Result;

fn main() -> Result<()> {
    let world = World::new(1900, 980)?;
    let app = WorldGenApp::new(world);
    let _ = eframe::run_native(
        "World Generation Biomes",
        eframe::NativeOptions::default(),
        Box::new(move |cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(app))
        }),
    );
    Ok(())
}
