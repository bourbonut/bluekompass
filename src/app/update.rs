use super::BlueKompassApp;

use egui_plot::{PlotUi, PlotPoint};

impl BlueKompassApp {
    fn update_shape(&mut self, shape_index: usize, pos: PlotPoint) {
        self.select_point_from_shape(shape_index, pos);
        if let Some(point_index) = self.selected_point_index {
            let shape = &mut self.shapes[shape_index];
            shape.replace(point_index, pos);
        }
    }

    pub fn move_selected_point(&mut self, plot_ui: &mut PlotUi) -> bool {
        if let Some(selected_index) = self.selected_shape_index {
            let response = plot_ui.response();
            if plot_ui.ctx().input(|i| i.pointer.primary_down()) {
                match plot_ui.pointer_coordinate() {
                    Some(pos) if response.contains_pointer() => {
                        self.update_shape(selected_index, pos);
                        return true;
                    }
                    _ => (),
                }
            } else {
                self.selected_point_index = None;
            }
        }
        false
    }
}
