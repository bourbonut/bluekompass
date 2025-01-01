use super::BlueKompassApp;
use egui_plot::PlotUi;

use crate::builders::BuilderMode;

impl BlueKompassApp {
    pub fn build(&mut self, plot_ui: &mut PlotUi, builder_mode: BuilderMode) {
        // Fix plot bounds and unselect current shape
        self.plot_bounds = plot_ui.plot_bounds();
        self.unselect_shape();

        // Build a shape or draw it
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

        // Set back plot bounds
        plot_ui.set_plot_bounds(self.plot_bounds);
    }
}
