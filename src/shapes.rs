use core::f32;
use std::f64::consts::TAU;
use egui_plot::{PlotPoint, PlotPoints, PlotUi, MarkerShape};
use eframe::{egui, epaint};
use egui::{remap, Vec2};

use crate::maths::{compute_circle_center, compute_circle_radius};

const BLUE: egui::Color32 = epaint::Color32::from_rgb(46, 101, 255);

pub trait Draw {
    fn draw(&self, plot_ui: &mut PlotUi);
}

pub trait Select {
    fn select_from_point(&self, point: Vec2) -> f32;
}

pub trait Shape: Draw + Select {
    fn select(&mut self);
    fn unselect(&mut self);
    fn as_slice(&self) -> &[PlotPoint];
    fn replace(&mut self, index: usize, point: PlotPoint);
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

impl Shape for Line {
    fn select(&mut self) {
        self.selected = true;
    }

    fn unselect(&mut self) {
        self.selected = false;
    }

    fn as_slice(&self) -> &[PlotPoint] {
        self.points.as_slice()
    }

    fn replace(&mut self, index: usize, point: PlotPoint) {
        self.points[index] = point;
    }
}

impl Draw for Line {
    fn draw(&self, plot_ui: &mut PlotUi) {
        let color = if self.selected { BLUE } else { epaint::Color32::BLACK };

        // Line
        plot_ui.line(
            egui_plot::Line::new(PlotPoints::Owned(self.points.to_vec()))
                .stroke(epaint::Stroke::new(3.0, epaint::Color32::BLACK))
        );

        // Stroke color for points
        plot_ui.points(
            egui_plot::Points::new(PlotPoints::Owned(self.points.to_vec()))
                .radius(6.0)
                .filled(true)
                .shape(MarkerShape::Circle)
                .color(epaint::Color32::WHITE)
        );

        // Fill color for points
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
    fn select_from_point(&self, point: Vec2) -> f32 {
        let [a, b] = self.points.map(|p| p.to_vec2());
        let ab = b - a;
        let ap = point - a;
        let k = ap.dot(ab) / ab.length_sq();
        if 0. <= k && k <= 1. { // point is between A and B
            // Distance between a point and the line
            return (ap.length_sq() - k * k * ab.length_sq()).sqrt();
        }
        f32::INFINITY
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
        let center = compute_circle_center(&points);
        let radius = compute_circle_radius(&center.to_vec2(), &points[0].to_vec2());
        Self { points, center, radius, selected: false }
    }
}

impl Shape for Circle {
    fn select(&mut self) {
        self.selected = true;
    }

    fn unselect(&mut self) {
        self.selected = false;
    }

    fn as_slice(&self) -> &[PlotPoint] {
        self.points.as_slice()
    }

    fn replace(&mut self, index: usize, point: PlotPoint) {
        self.points[index] = point;
        self.center = compute_circle_center(&self.points);
        self.radius = compute_circle_radius(&self.center.to_vec2(), &self.points[0].to_vec2());
    }
}

impl Draw for Circle {
    fn draw(&self, plot_ui: &mut PlotUi) {
        let color = if self.selected { BLUE } else { epaint::Color32::BLACK };
        let radius = self.radius as f64;
        let n = 512;

        // Circle
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
            ).stroke(epaint::Stroke::new(3.0, epaint::Color32::BLACK))
        );

        // Stroke color for points
        plot_ui.points(
            egui_plot::Points::new(PlotPoints::Owned(self.points.to_vec()))
                .radius(6.0)
                .filled(true)
                .shape(MarkerShape::Circle)
                .color(epaint::Color32::WHITE)
        );

        // Fill color for points
        plot_ui.points(
            egui_plot::Points::new(PlotPoints::Owned(self.points.to_vec()))
                .radius(5.0)
                .filled(true)
                .shape(MarkerShape::Circle)
                .color(color)
        );

        // Center of circle
        plot_ui.points(
            egui_plot::Points::new(PlotPoints::Owned(vec![self.center]))
                .radius(5.0)
                .filled(true)
                .shape(MarkerShape::Cross)
                .color(epaint::Color32::BLACK)
        );
    }
}

impl Select for Circle {
    fn select_from_point(&self, point: Vec2) -> f32 {
        let radius2 = self.radius * self.radius;
        // circle equation ^ 2 = radius ^ 2
        // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
        (((point - self.center.to_vec2()).length_sq() - radius2) / self.radius).abs()
    }
}
