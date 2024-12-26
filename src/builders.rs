use egui_plot::{PlotPoints, PlotPoint, PlotUi, MarkerShape};
use eframe::epaint;
use egui_plot;

use crate::shapes::{Circle, Line, Draw};

pub trait Build {
    type Output;

    fn build(&self) -> Option<Self::Output>;
}

pub trait DrawWithPoint {
    fn draw(&self, plot_ui: &mut PlotUi, current_point: PlotPoint);
}

#[derive(Debug, PartialEq)]
pub struct LineBuilder {
    p1: Option<PlotPoint>,
    p2: Option<PlotPoint>,
}

impl LineBuilder {
    pub fn new() -> Self {
        Self { p1: None, p2: None }
    }

    pub fn set_point(&mut self, point: PlotPoint) {
        if self.p1.is_none() {
            self.p1 = Some(point);
            return;
        }
        self.p2 = Some(point);
    }
}

impl Build for LineBuilder {
    type Output = Line;

    fn build(&self) -> Option<Self::Output> {
        if self.p1.is_none() || self.p2.is_none() {
            return None;
        } 
        let p1 = self.p1.unwrap();
        let p2 = self.p2.unwrap();
        Some(Line::new([p1, p2]))
    }
}

impl DrawWithPoint for LineBuilder {
    fn draw(&self, plot_ui: &mut PlotUi, current_point: PlotPoint) {
        if let Some(p1) = self.p1 {
            let line = Line::new([p1, current_point]);
            line.draw(plot_ui);
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CircleBuilder {
    p1: Option<PlotPoint>,
    p2: Option<PlotPoint>,
    p3: Option<PlotPoint>,
}

impl CircleBuilder {
    pub fn new() -> Self {
        Self { p1: None, p2: None, p3: None }
    }

    pub fn set_point(&mut self, point: PlotPoint) {
        if self.p1.is_none() {
            self.p1 = Some(point);
            return;
        } else if self.p2.is_none() {
            self.p2 = Some(point);
            return;
        }
        self.p3 = Some(point);
    }
}

impl Build for CircleBuilder {
    type Output = Circle;

    fn build(&self) -> Option<Self::Output> {
        if self.p1.is_none() || self.p2.is_none() || self.p3.is_none() {
            return None;
        }
        let p1 = self.p1.unwrap();
        let p2 = self.p2.unwrap();
        let p3 = self.p3.unwrap();
        Some(Circle::new([p1, p2, p3]))
    }
}

impl DrawWithPoint for CircleBuilder {
    fn draw(&self, plot_ui: &mut PlotUi, current_point: PlotPoint) {
        if let Some(p1) = self.p1 {
            if let Some(p2) = self.p2 {
                let circle = Circle::new([p1, p2, current_point]);
                //println!("Partial circle: {:?}", circle);
                circle.draw(plot_ui);
            } else {
                plot_ui.points(
                    egui_plot::Points::new(PlotPoints::Owned(vec![p1]))
                        .radius(6.0)
                        .filled(true)
                        .shape(MarkerShape::Circle)
                        .color(epaint::Color32::WHITE)
                );
                plot_ui.points(
                    egui_plot::Points::new(PlotPoints::Owned(vec![p1]))
                        .radius(5.0)
                        .filled(true)
                        .shape(MarkerShape::Circle)
                        .color(epaint::Color32::BLACK)
                );
            }
        }
    }
}

#[derive(PartialEq)]
pub enum Builder {
    NoneBuilder,
    Line(LineBuilder),
    Circle(CircleBuilder),
}
