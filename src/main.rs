#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod image_loader;
use self::app::MasonApp;
use eframe;

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let icon = include_bytes!("../assets/mason.png");
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_icon(
            eframe::icon_data::from_png_bytes(&icon[..]).unwrap()
        ),
        ..Default::default()
    };
    eframe::run_native(
        "Mason Application",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MasonApp>::default()
        }),
    )
}
