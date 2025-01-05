use super::image_loader::BlueKompassImage;
use eframe::egui;
use egui_plot::{Plot, PlotBounds};

use egui::Layout;

use egui_file::FileDialog;
use std::path::PathBuf;

use crate::shapes::Shape;
use crate::builders::{Builder, Line, Circle};

mod selection;
mod build;
mod image;
mod draw;
mod update;
mod remove;

#[derive(PartialEq)]
enum Mode {
    DRAG,
    SELECTION,
    LINE,
    CIRCLE,
    //SPLINE,
}

const MODES: [(Mode, &str); 4] = [
    (Mode::DRAG, "Drag"),
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
    selected_shape_index: Option<usize>,
    selected_point_index: Option<usize>,
}

impl Default for BlueKompassApp {
    fn default() -> Self {
        Self {
            image: None,
            mode: Mode::DRAG,
            opened_file: None,
            open_file_dialog: None,
            builder: Builder::new(),
            shapes: Vec::default(),
            plot_bounds: PlotBounds::from_min_max([0., 0.], [0., 0.]),
            selected_shape_index: None,
            selected_point_index: None,
        }
    }
}

impl eframe::App for BlueKompassApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Define layout
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
                        self.builder.reset();
                    }
                }
            });


            self.refresh_image(ctx);

            if let Some(image) = &mut self.image {
                let (image_id, size) = image.load(ui);

                let plot = Plot::new("BlueKompass Plot")
                    .data_aspect(1.0)
                    .allow_drag(self.mode != Mode::SELECTION)
                    .show_axes(false)
                    .show_x(false)
                    .show_y(false)
                    .show_grid(false);

                plot.show(ui, |plot_ui| {
                    self.draw_image(plot_ui, image_id, size);

                    match self.mode {
                        Mode::DRAG => self.unselect_shape(),
                        Mode::SELECTION => self.select(plot_ui),
                        Mode::LINE => self.build(plot_ui, Line),
                        Mode::CIRCLE => self.build(plot_ui, Circle),
                    }

                    self.draw(plot_ui);
                });
            } else {
                ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                    ui.label("Welcome to BlueKompass v0.1.0 !\nStart by opening an image from the \"Open\" menu.");
                });
            }
        });
    }
}
