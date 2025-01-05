use egui_plot::{PlotPoint, PlotUi};

use crate::shapes::{self, Shape, Draw};
use super::ShapeBuilder;

pub struct Line;

impl ShapeBuilder for Line { 
    fn build(&self, points: &Vec<PlotPoint>) -> Option<Box<dyn Shape>> {
        if points.len() < 2 {
            return None;
        }
        Some(Box::new(shapes::Line::new([points[0], points[1]])))
    }


    fn draw(&self, points: &Vec<PlotPoint>, plot_ui: &mut PlotUi, current_point: PlotPoint) {
        if points.len() > 0 {
            let line = shapes::Line::new([points[0], current_point]);
            line.draw(plot_ui);
        }
    }
}

