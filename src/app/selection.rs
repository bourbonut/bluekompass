use super::BlueKompassApp;
use std::cmp::Ordering;

use eframe::egui;
use egui_plot::{PlotPoint, PlotUi};

impl BlueKompassApp {
    fn remove_shape(&mut self) {
        if let Some(selection_index) = self.selected_shape_index {
            self.selected_point_index = None;
            self.shapes.remove(selection_index);
            self.selected_shape_index = None;
        }
    }

    fn remove_selected_shape(&mut self, plot_ui: &mut PlotUi) -> bool {
        if plot_ui.ctx().input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::D)) {
            self.remove_shape();
            return true;
        }
        false
    }

    fn select_shape(&mut self, selection_index: usize) {
        self.shapes[selection_index].select();
        self.selected_shape_index = Some(selection_index);
    }

    fn select_point_from_shape(&mut self, shape_index: usize, pos: PlotPoint){
        if self.selected_point_index.is_none() {
            let pos = pos.to_vec2();
            let shape = &self.shapes[shape_index];
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
                    self.selected_point_index = Some(point_index);
                }
                _ => (),
            }
        }
    }

    fn update_shape(&mut self, shape_index: usize, pos: PlotPoint) {
        self.select_point_from_shape(shape_index, pos);
        if let Some(point_index) = self.selected_point_index {
            let shape = &mut self.shapes[shape_index];
            shape.replace(point_index, pos);
        }
    }

    fn move_selected_point(&mut self, plot_ui: &mut PlotUi) -> bool {
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

    pub fn unselect_shape(&mut self) {
        if let Some(selection_index) = self.selected_shape_index {
            self.shapes[selection_index].unselect();
            self.selected_shape_index = None;
        }
    }

    pub fn select(&mut self, plot_ui: &mut PlotUi) {
        if self.remove_selected_shape(plot_ui) {
            return;
        }
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
