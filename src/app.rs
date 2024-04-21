use super::image_loader::MasonImage;
use eframe::egui::{self, Vec2};
use egui_plot::{Plot, PlotImage, PlotPoint};

use egui_file::FileDialog;
use std::path::{PathBuf, Path};
use std::ffi::OsStr;

pub struct MasonApp {
    image: Option<MasonImage>,
    opened_file: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
}

impl Default for MasonApp {
    fn default() -> Self {
        Self { image: None, opened_file: None, open_file_dialog: None }
    }
}

impl eframe::App for MasonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if (ui.button("Open")).clicked() {
                // Show only files with the extension "png".
                let filter = Box::new({
                    let ext = Some(OsStr::new("png"));
                    move |path: &Path| -> bool { path.extension() == ext }
                });
                let mut dialog = FileDialog::open_file(self.opened_file.clone()).show_files_filter(filter);
                dialog.open();
                self.open_file_dialog = Some(dialog);
            }

            if let Some(dialog) = &mut self.open_file_dialog {
                if dialog.show(ctx).selected() {
                    if let Some(file) = dialog.path() {
                        self.opened_file = Some(file.to_path_buf());
                        self.image = Some(MasonImage::new(file.to_path_buf()))
                    }
                }
            }

            if let Some(image) = &mut self.image {
                let mut plot = Plot::new("Mason Plot")
                    .show_axes(false)
                    .show_x(false)
                    .show_y(false)
                    .show_grid(false);
                let image_id = image.load(ui);
                plot = plot.data_aspect(1.0);
                plot.show(ui, |plot_ui| {
                    plot_ui.image(
                        PlotImage::new(
                            image_id,
                            PlotPoint::new(0.0, 0.0),
                            Vec2::new(1788.0, 1036.0) // TODO: make it dynamic
                        )
                    )
                });
            }
        });
    }
}
