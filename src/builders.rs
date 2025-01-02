use egui_plot::{PlotPoints, PlotPoint, PlotUi, MarkerShape};
use eframe::epaint;
use egui_plot;

use crate::shapes::{Circle, Line, Draw, Shape};


fn build_line(points: [Option<&PlotPoint>; 2]) -> Option<Box<dyn Shape>> {
    let [p1, p2] = points;
    if p1.is_none() || p2.is_none() {
        return None;
    } 
    let p1 = p1.unwrap();
    let p2 = p2.unwrap();
    Some(Box::new(Line::new([*p1, *p2])))
}


fn partial_draw_line(p: Option<&PlotPoint>, plot_ui: &mut PlotUi, current_point: PlotPoint) {
    if let Some(p1) = p {
        let line = Line::new([*p1, current_point]);
        line.draw(plot_ui);
    }
}

fn build_circle(points: [Option<&PlotPoint>; 3]) -> Option<Box<dyn Shape>> {
    let [p1, p2, p3] = points;
    if p1.is_none() || p2.is_none() || p3.is_none() {
        return None;
    }
    let p1 = p1.unwrap();
    let p2 = p2.unwrap();
    let p3 = p3.unwrap();
    Some(Box::new(Circle::new([*p1, *p2, *p3])))
}


fn partial_draw_circle(points: [Option<&PlotPoint>; 2], plot_ui: &mut PlotUi, current_point: PlotPoint) {
    if let Some(p1) = points[0] {
        if let Some(p2) = points[1] {
            let circle = Circle::new([*p1, *p2, current_point]);
            circle.draw(plot_ui);
        } else {
            plot_ui.points(
                egui_plot::Points::new(PlotPoints::Owned(vec![*p1]))
                    .radius(6.0)
                    .filled(true)
                    .shape(MarkerShape::Circle)
                    .color(epaint::Color32::WHITE)
            );
            plot_ui.points(
                egui_plot::Points::new(PlotPoints::Owned(vec![*p1]))
                    .radius(5.0)
                    .filled(true)
                    .shape(MarkerShape::Circle)
                    .color(epaint::Color32::BLACK)
            );
        }
    }
}


#[derive(PartialEq)]
pub enum BuilderMode {
    Line,
    Circle,
}

pub struct Builder {
    points: Vec<PlotPoint>,
    mode: BuilderMode,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            mode: BuilderMode::Line,
        }
    }

    pub fn reset(&mut self) {
        self.points.clear();
    }

    pub fn set_mode(&mut self, mode: BuilderMode) {
        if self.mode != mode {
            self.reset();
            self.mode = mode;
        }
    }

    pub fn set_next_point(&mut self, point: PlotPoint) {
        if self.points.len() < 3 {
            self.points.push(point);
        }
    }

    pub fn draw(&self, plot_ui: &mut PlotUi, current_point: PlotPoint) {
        match self.mode {
            BuilderMode::Line => partial_draw_line(self.points.get(0), plot_ui, current_point),
            BuilderMode::Circle => partial_draw_circle([self.points.get(0), self.points.get(1)], plot_ui, current_point),
        }
    }

    pub fn build(&self) -> Option<Box<dyn Shape>> {
        match self.mode {
            BuilderMode::Line => build_line([self.points.get(0), self.points.get(1)]),
            BuilderMode::Circle => build_circle([self.points.get(0), self.points.get(1), self.points.get(2)]),
        }
    }
}
