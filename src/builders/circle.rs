use egui_plot::{MarkerShape, PlotPoints, PlotPoint, PlotUi};
use eframe::epaint;

use crate::shapes::{self, Shape, Draw};
use super::ShapeBuilder;

pub struct Circle;

impl ShapeBuilder for Circle {  
    fn build(&self, points: &Vec<PlotPoint>) -> Option<Box<dyn Shape>> {
        if points.len() < 3 {
            return None;
        }
        Some(Box::new(shapes::Circle::new([points[0], points[1], points[2]])))
    }


    fn draw(&self, points: &Vec<PlotPoint>, plot_ui: &mut PlotUi, current_point: PlotPoint) {
        if points.len() > 0 {
            if points.len() > 1 {
                let circle = shapes::Circle::new([points[0], points[1], current_point]);
                circle.draw(plot_ui);
            } else {
                plot_ui.points(
                    egui_plot::Points::new(PlotPoints::Owned(points.to_vec()))
                        .radius(6.0)
                        .filled(true)
                        .shape(MarkerShape::Circle)
                        .color(epaint::Color32::WHITE)
                );
                plot_ui.points(
                    egui_plot::Points::new(PlotPoints::Owned(points.to_vec()))
                        .radius(5.0)
                        .filled(true)
                        .shape(MarkerShape::Circle)
                        .color(epaint::Color32::BLACK)
                );
            }
        }
    }
}

