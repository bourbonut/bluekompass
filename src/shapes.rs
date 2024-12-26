use std::f64::consts::TAU;
use egui_plot::{PlotPoint, PlotPoints, PlotUi, MarkerShape};
use eframe::{egui, epaint};
use egui::{remap, Vec2};


pub trait Draw {
    fn draw(&self, plot_ui: &mut PlotUi);
}

trait Select {
    fn select_from_point(&self, point: Vec2) -> bool;
}

#[derive(Debug)]
pub struct Line {
    points: [PlotPoint; 2],
    selected: bool,
}

impl Line {
    pub fn new(points: [PlotPoint; 2]) -> Self {
        Self { points, selected: false }
    }
}

impl Draw for Line {
    fn draw(&self, plot_ui: &mut PlotUi) {
        let color = if self.selected { epaint::Color32::GREEN } else { epaint::Color32::BLACK };
        plot_ui.line(
            egui_plot::Line::new(PlotPoints::Owned(self.points.to_vec()))
                .stroke(epaint::Stroke::new(3.0, color))
        );
        plot_ui.points(
            egui_plot::Points::new(PlotPoints::Owned(self.points.to_vec()))
                .radius(6.0)
                .filled(true)
                .shape(MarkerShape::Circle)
                .color(epaint::Color32::WHITE)
        );
        plot_ui.points(
            egui_plot::Points::new(PlotPoints::Owned(self.points.to_vec()))
                .radius(5.0)
                .filled(true)
                .shape(MarkerShape::Circle)
                .color(color)
        );
    }
}

impl Select for Line {
    fn select_from_point(&self, _point: Vec2) -> bool {
        // TODO: complete this trait
        let vectors = self.points.map(|p| p.to_vec2());
        let _p1 = vectors[0];
        let _p2 = vectors[1];

        false
    }
}

#[derive(Debug)]
pub struct Circle {
    points: [PlotPoint; 3],
    center: PlotPoint,
    radius: f32,
    selected: bool,
}

impl Circle {
    pub fn new(points: [PlotPoint; 3]) -> Self {
        let center = Circle::compute_center(&points);
        let radius = Circle::compute_radius(&center.to_vec2(), &points[0].to_vec2());
        Self { points, center, radius, selected: false }
    }

    fn compute_center(points: &[PlotPoint; 3]) -> PlotPoint {
        // TODO : Use Vec2 for simpler expressions and better readability
        let vectors = points.map(|p| p.to_vec2());

        let p1 = vectors[0];
        let p2 = vectors[1];
        let p3 = vectors[2];

        let x1 = p1.x;
        let y1 = p1.y;
        let x2 = p2.x;
        let y2 = p2.y;
        let x3 = p3.x;
        let y3 = p3.y;

        let denom = 2. * (x1 * y2 - x1 * y3 - x2 * y1 + x2 * y3 + x3 * y1 - x3 * y2);

        let cx = (
            (x1 * x1 + y1 * y1) * (y2 - y3)
            - (x2 * x2 + y2 * y2) * (y1 - y3)
            + (x3 * x3 + y3 * y3) * (y1 - y2)
        ) / denom;

        let cy = (
            -(x1 - x2) * (x3 * x3 + y3 * y3)
            + (x1 - x3) * (x2 * x2 + y2 * y3)
            - (x1 * x1 + y1 * y1) * (x2 - x3)
        ) / denom;

        PlotPoint::new(cx, cy)
    }

    fn compute_radius(center: &Vec2, circle_point: &Vec2) -> f32 {
        (*center - *circle_point).length()
    }
}

impl Draw for Circle {
    fn draw(&self, plot_ui: &mut PlotUi) {
        let color = if self.selected { epaint::Color32::GREEN } else { epaint::Color32::BLACK };
        let radius = self.radius as f64;
        let n = 512;
        plot_ui.line(
            egui_plot::Line::new(
                (0..=n).map(
                    |i| {
                        let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                        [
                            radius * t.cos() + self.center.x,
                            radius * t.sin() + self.center.y,
                        ]
                    }
                ).collect::<PlotPoints>()
            ).stroke(epaint::Stroke::new(3.0, color))
        );
        plot_ui.points(
            egui_plot::Points::new(PlotPoints::Owned(self.points.to_vec()))
                .radius(6.0)
                .filled(true)
                .shape(MarkerShape::Circle)
                .color(epaint::Color32::WHITE)
        );
        plot_ui.points(
            egui_plot::Points::new(PlotPoints::Owned(self.points.to_vec()))
                .radius(5.0)
                .filled(true)
                .shape(MarkerShape::Circle)
                .color(color)
        );
    }
}

impl Select for Circle {
    fn select_from_point(&self, _point: Vec2) -> bool {
        // TODO: complete this trait
        let vectors = self.points.map(|p| p.to_vec2());
        let _p1 = vectors[0];
        let _p2 = vectors[1];
        let _p3 = vectors[2];

        false
    }
}

pub enum Shape {
    Line(Line),
    Circle(Circle),
}
