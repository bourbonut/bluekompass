use egui_plot::{PlotPoints, PlotPoint, PlotUi, MarkerShape};
use eframe::epaint;
use egui_plot;

use crate::shapes::{self, Draw, Shape};

pub struct Line;

pub trait ShapeBuilder {
    fn build(&self, points: &Vec<PlotPoint>) -> Option<Box<dyn Shape>>;
    fn draw(&self, points: &Vec<PlotPoint>, plot_ui: &mut PlotUi, current_point: PlotPoint);
}

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


pub struct Builder {
    points: Vec<PlotPoint>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.points.clear();
    }

    pub fn set_next_point(&mut self, point: PlotPoint) {
        if self.points.len() < 3 {
            self.points.push(point);
        }
    }

    pub fn draw<T: ShapeBuilder>(&self, plot_ui: &mut PlotUi, current_point: PlotPoint, shape: T) { 
        shape.draw(&self.points, plot_ui, current_point);
    }

    pub fn build<T: ShapeBuilder>(&self, shape: T) -> Option<Box<dyn Shape>> {
        shape.build(&self.points)
    }
}
