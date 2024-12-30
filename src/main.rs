#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod image_loader;
mod shapes;
mod builders;
mod maths;
use self::app::BlueKompassApp;
use eframe;

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let icon = include_bytes!("../assets/bluekompass.png");
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_icon(
            eframe::icon_data::from_png_bytes(&icon[..]).unwrap()
        ),
        ..Default::default()
    };
    eframe::run_native(
        "BlueKompass Application",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<BlueKompassApp>::default()
        }),
    )
}
