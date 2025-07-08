use betago::gui::app::GoApp;
use egui::{Vec2, ViewportBuilder};

fn main() -> eframe::Result<()> {
    let viewport_builder = ViewportBuilder {
        resizable: Some(false),
        inner_size: Some(Vec2::new(410.0, 410.0)),
        ..Default::default()
    };
    let native_options = eframe::NativeOptions {
        viewport: viewport_builder,
        ..Default::default()
    };

    eframe::run_native(
        "Go game",
        native_options,
        Box::new(|_cc| Ok(Box::new(GoApp::default()))),
    )
}
