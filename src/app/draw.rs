use super::BlueKompassApp;

use egui_plot::PlotUi;

impl BlueKompassApp {
    pub fn draw(&mut self, plot_ui: &mut PlotUi) {
        for shape in &self.shapes {
            shape.draw(plot_ui);
        }
    }
}
