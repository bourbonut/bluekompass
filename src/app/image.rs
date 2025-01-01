use super::{BlueKompassApp, Mode, BlueKompassImage};

use eframe::egui;
use egui_plot::{PlotImage, PlotPoint, PlotUi};
use egui::{Context, TextureId, Vec2};
use std::ffi::OsStr;
use std::path::Path;
use egui_file::FileDialog;

impl BlueKompassApp {
    pub fn open_image(&mut self) {
        // Show only files with the extension "png".
        let filter = Box::new({
            let ext = Some(OsStr::new("png"));
            move |path: &Path| -> bool { path.extension() == ext }
        });
        let mut dialog = FileDialog::open_file(self.opened_file.clone()).show_files_filter(filter);
        dialog.open();
        self.open_file_dialog = Some(dialog);
        self.mode = Mode::SELECTION;
    }

    pub fn refresh_image(&mut self, ctx: &Context) {
        if let Some(dialog) = &mut self.open_file_dialog {
            if dialog.show(ctx).selected() {
                if let Some(file) = dialog.path() {
                    self.opened_file = Some(file.to_path_buf());
                    self.image = Some(BlueKompassImage::new(file.to_path_buf()))
                }
            }
        }
    }

    pub fn draw_image(&mut self, plot_ui: &mut PlotUi, image_id: TextureId, size: [usize; 2]) {
        plot_ui.image(
            PlotImage::new(
                image_id,
                PlotPoint::new(0.0, 0.0),
                Vec2::new(size[0] as f32, size[1] as f32)
            )
        );
    }
}
