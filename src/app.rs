use super::image_loader::BlueKompassImage;
use eframe::egui;
use egui_plot::{Plot, PlotBounds, PlotImage, PlotPoint, PlotUi};

use egui::{Context, Layout, TextureId, Vec2};

use egui_file::FileDialog;
use std::path::{PathBuf, Path};
use std::ffi::OsStr;
use std::cmp::Ordering;

use crate::shapes::Shape;
use crate::builders::{Builder, BuilderMode};

#[derive(PartialEq)]
enum Mode {
    SELECTION,
    LINE,
    CIRCLE,
    //SPLINE,
}

const MODES: [(Mode, &str); 3] = [
    (Mode::SELECTION, "Selection"),
    (Mode::LINE, "Line"),
    (Mode::CIRCLE, "Circle"),
    //(Mode::SPLINE, "Spline"),
];

pub struct BlueKompassApp {
    image: Option<BlueKompassImage>,
    mode: Mode,
    opened_file: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
    builder: Builder,
    shapes: Vec<Box<dyn Shape>>,
    plot_bounds: PlotBounds,
    last_selection: Option<usize>,
}

impl Default for BlueKompassApp {
    fn default() -> Self {
        Self {
            image: None,
            mode: Mode::SELECTION,
            opened_file: None,
            open_file_dialog: None,
            builder: Builder::new(),
            shapes: Vec::default(),
            plot_bounds: PlotBounds::from_min_max([0., 0.], [0., 0.]),
            last_selection: None,
        }
    }
}

impl BlueKompassApp {
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
                    self.image = Some(BlueKompassImage::new(file.to_path_buf()))
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

    fn build(&mut self, plot_ui: &mut PlotUi, builder_mode: BuilderMode) {
        self.builder.set_mode(builder_mode);
        let response = plot_ui.response();
        if plot_ui.ctx().input(|i| i.pointer.primary_clicked()) {
            if response.contains_pointer() {
                if let Some(pos) = plot_ui.pointer_coordinate() {
                    self.builder.set_next_point(pos);
                    if let Some(shape) = self.builder.build() {
                        self.shapes.push(shape);
                        self.builder.reset();
                    } 
                }
            }
        } else if let Some(pos) = plot_ui.pointer_coordinate() {
            if response.contains_pointer() {
                self.builder.draw(plot_ui, pos);
            }
        }
    }

    fn draw(&mut self, plot_ui: &mut PlotUi) {
        for shape in &self.shapes {
            shape.draw(plot_ui);
        }
    }

    fn unselect_shape(&mut self) {
        if let Some(selection_index) = self.last_selection {
            self.shapes[selection_index].unselect();
            self.last_selection = None;
        }
    }

    fn select_shape(&mut self, selection_index: usize) {
        self.shapes[selection_index].select();
        self.last_selection = Some(selection_index);
    }

    fn update_shape(&mut self, shape_index: usize, pos: PlotPoint) {
        let shape = &mut self.shapes[shape_index];
        let pos = pos.to_vec2();
        let result = shape.as_slice()
            .iter()
            .enumerate()
            .map(|(i, point)| (i, (point.to_vec2() - pos).length()))
            .min_by(
                |(_, r1), (_, r2)| {
                    r1.partial_cmp(&r2)
                        .unwrap_or(Ordering::Equal)
                }
            );
        match result {
            Some((point_index, radius)) if radius < 10. => {
                shape.replace(point_index, PlotPoint::new(pos.x, pos.y));
            }
            _ => (),
        }
    }

    fn move_selected_point(&mut self, plot_ui: &mut PlotUi) -> bool {
        if let Some(selected_index) = self.last_selection {
            let response = plot_ui.response();
            if plot_ui.ctx().input(|i| i.pointer.primary_down()) {
                match plot_ui.pointer_coordinate() {
                    Some(pos) if response.contains_pointer() => {
                        self.update_shape(selected_index, pos);
                        return true;
                    }
                    _ => (),
                }
            } 
        }
        false
    }

    fn select_next_shape(&mut self, pos: PlotPoint) {
        let pos = pos.to_vec2();
        let result = self.shapes.iter()
            .enumerate()
            .map(|(i, shape)| (i, shape.select_from_point(pos)))
            .min_by(
                |(_, score_a), (_, score_b)| {
                    score_a.partial_cmp(&score_b)
                        .unwrap_or(Ordering::Equal)
                }
            );
        match result {
            Some((selection_index, score)) if score < 10. => {
                self.unselect_shape();
                self.select_shape(selection_index);
            }
            Some(_) => self.unselect_shape(),
            None => (),
        }
    }

    fn select(&mut self, plot_ui: &mut PlotUi) {
        if self.move_selected_point(plot_ui) {
            return;
        }
        let response = plot_ui.response();
        if plot_ui.ctx().input(|i| i.pointer.primary_clicked()) {
            match plot_ui.pointer_coordinate() {
                Some(pos) if response.contains_pointer() => {
                    self.select_next_shape(pos);
                }
                _ => (),
            }
        }
    }
}

impl eframe::App for BlueKompassApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Layout::default().horizontal_align()), |ui|{
                // Add "Open" button to open images
                if ui.button("Open").clicked() {
                    self.open_image();
                }

                // Select mode with buttons
                for (mode, button_text) in MODES {
                    // ui.add(Button::image(Image::new(include_image!("../assets/bluekompass.png"))));
                    if ui.button(button_text).clicked() {
                        self.mode = mode;
                    }
                }
            });

            self.refresh_image(ctx);

            if let Some(image) = &mut self.image {
                let (image_id, size) = image.load(ui);
                let plot = Plot::new("BlueKompass Plot")
                    .data_aspect(1.0)
                    .allow_drag(self.mode != Mode::SELECTION) // TODO: find a solution to remove
                    // this line
                    .show_axes(false)
                    .show_x(false)
                    .show_y(false)
                    .show_grid(false);
                plot.show(ui, |plot_ui| {
                    self.draw_image(plot_ui, image_id, size);


                    match self.mode {
                        Mode::SELECTION => self.select(plot_ui),
                        Mode::LINE => self.build(plot_ui, BuilderMode::Line),
                        Mode::CIRCLE => {
                            self.plot_bounds = plot_ui.plot_bounds();
                            self.build(plot_ui, BuilderMode::Circle);
                            plot_ui.set_plot_bounds(self.plot_bounds);
                        },
                    }

                    self.draw(plot_ui);
                });
            }
        });
    }
}
