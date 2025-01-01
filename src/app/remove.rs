use super::BlueKompassApp;

use egui_plot::PlotUi;
use eframe::egui;


impl BlueKompassApp {
    fn remove_shape(&mut self) {
        if let Some(selection_index) = self.selected_shape_index {
            self.selected_point_index = None;
            self.shapes.remove(selection_index);
            self.selected_shape_index = None;
        }
    }

    pub fn remove_selected_shape(&mut self, plot_ui: &mut PlotUi) -> bool {
        if plot_ui.ctx().input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::D)) {
            self.remove_shape();
            return true;
        }
        false
    }
}
