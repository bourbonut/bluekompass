use super::image_loader::MasonImage;
use eframe::egui;
use egui_plot::{Plot, PlotImage, PlotPoint, PlotPoints, PlotUi, Points};

use egui::{Context, Layout, TextureId, Vec2};

use egui_file::FileDialog;
use std::path::{PathBuf, Path};
use std::ffi::OsStr;

#[derive(PartialEq)]
enum Mode {
    SELECTION,
    POINT,
    LINE,
    CIRCLE,
    SPLINE,
}

const MODES: [(Mode, &str); 5] = [
    (Mode::SELECTION, "Selection"),
    (Mode::POINT, "Point"),
    (Mode::LINE, "Line"),
    (Mode::CIRCLE, "Circle"),
    (Mode::SPLINE, "Spline"),
];

pub struct MasonApp {
    image: Option<MasonImage>,
    mode: Mode,
    opened_file: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
    points: Vec<PlotPoint>,
}

impl Default for MasonApp {
    fn default() -> Self {
        Self { image: None, mode: Mode::SELECTION, opened_file: None, open_file_dialog: None, points: Vec::default() }
    }
}

impl MasonApp {
    fn open_image(&mut self) {
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

    fn refresh_image(&mut self, ctx: &Context) {
        if let Some(dialog) = &mut self.open_file_dialog {
            if dialog.show(ctx).selected() {
                if let Some(file) = dialog.path() {
                    self.opened_file = Some(file.to_path_buf());
                    self.image = Some(MasonImage::new(file.to_path_buf()))
                }
            }
        }
    }

    fn draw_image(&mut self, plot_ui: &mut PlotUi, image_id: TextureId, size: [usize; 2]) {
        plot_ui.image(
            PlotImage::new(
                image_id,
                PlotPoint::new(0.0, 0.0),
                Vec2::new(size[0] as f32, size[1] as f32)
            )
        );
    }

    fn add_point(&mut self, plot_ui: &mut PlotUi){
        let response = plot_ui.response();
        if plot_ui.ctx().input(|i| i.pointer.primary_clicked()) {
            if response.contains_pointer() {
                if let Some(pos) =  plot_ui.pointer_coordinate() {
                    self.points.push(pos)
                }
            }
        }
    }

    fn remove_point(&mut self, plot_ui: &mut PlotUi){
        if plot_ui.ctx().input(|i| i.pointer.secondary_pressed()) && self.points.len() > 0 {
            if let Some(pos) =  plot_ui.pointer_coordinate() {
                let vpos = pos.to_vec2();
                let (index, min) = self.points.iter()
                    .enumerate()
                    .map(|(i, &p)| (i, (p.to_vec2() - vpos).length()))
                    .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                    .unwrap();
                if min < 10.0 {
                    self.points.remove(index);
                }
            }
        }
    }

    fn draw_points(&mut self, plot_ui: &mut PlotUi){
        plot_ui.points(
            Points::new(PlotPoints::Owned(self.points.clone()))
                .radius(10.0)
                .filled(true)
                .shape(egui_plot::MarkerShape::Cross)
        );
    }
}

impl eframe::App for MasonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Layout::default().horizontal_align()), |ui|{
                // Add buttons
                if ui.button("Open").clicked() {
                    self.open_image();
                }

                for (mode, button_text) in MODES {
                    // ui.add(Button::image(Image::new(include_image!("../assets/mason.png"))));
                    if ui.button(button_text).clicked() {
                        self.mode = mode;
                    }

                }
            });

            self.refresh_image(ctx);

            if let Some(image) = &mut self.image {
                let mut plot = Plot::new("Mason Plot")
                    .show_axes(false)
                    .show_x(false)
                    .show_y(false)
                    .show_grid(false);
                let (image_id, size) = image.load(ui);
                plot = plot.data_aspect(1.0);
                plot.show(ui, |plot_ui| {
                    self.draw_image(plot_ui, image_id, size);
                    self.draw_points(plot_ui);

                    if self.mode == Mode::POINT {
                        self.add_point(plot_ui);
                        self.remove_point(plot_ui);
                    }
                    // if let Some(coord) = plot_ui.pointer_coordinate() {
                    //     dbg!(
                    //         plot_ui.screen_from_plot(coord)
                    //     );
                    // }
                    // dbg!(plot_ui.response());
                });
            }
        });
    }
}
