use std::f64::consts::TAU;
use egui_plot::{PlotPoint, PlotPoints, PlotUi, MarkerShape};
use eframe::{egui, epaint};
use egui::{remap, Vec2};

pub trait Draw {
    fn draw(&self, plot_ui: &mut PlotUi);
}

#[derive(Debug)]
struct Line {
    points: [PlotPoint; 2],
}

impl Draw for Line {
    fn draw(&self, plot_ui: &mut PlotUi) {
        plot_ui.line(
            egui_plot::Line::new(PlotPoints::Owned(self.points.to_vec()))
                .stroke(epaint::Stroke::new(3.0, epaint::Color32::BLACK))
        );
        plot_ui.points(
            egui_plot::Points::new(PlotPoints::Owned(self.points.to_vec()))
                .radius(10.0)
                .filled(true)
                .shape(MarkerShape::Circle)
                .color(epaint::Color32::BLACK)
        );
    }
}

#[derive(Debug)]
struct Circle {
    points: [PlotPoint; 3],
    center: Option<PlotPoint>,
    radius: f32,
}

impl Circle {
    fn compute_center(&self) -> PlotPoint {
        // TODO : Use Vec2 for simpler expressions and better readability
        let vectors = self.points.map(|p| p.to_vec2());

        let p1 = vectors[0];
        let p2 = vectors[2];
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

    fn compute_radius(&self, center: Vec2) -> f32 {
        (center - self.points[0].to_vec2()).length()
    }

    fn set_center(&mut self, center: PlotPoint) {
        self.center = Some(center);
        self.radius = self.compute_radius(center.to_vec2());
    }
}

impl Draw for Circle {
    fn draw(&self, plot_ui: &mut PlotUi) {
        if let Some(center) = self.center {
                let radius = self.radius as f64;
                let n = 512;
                plot_ui.line(
                    egui_plot::Line::new(
                        (0..n).map(
                            |i| {
                                let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                                [
                                    radius * t.cos() + center.x,
                                    radius * t.sin() + center.y,
                                ]
                            }
                        ).collect::<PlotPoints>()
                    ).stroke(epaint::Stroke::new(3.0, epaint::Color32::BLACK))
                );
                plot_ui.points(
                    egui_plot::Points::new(PlotPoints::Owned(self.points.to_vec()))
                        .radius(10.0)
                        .filled(true)
                        .shape(MarkerShape::Circle)
                        .color(epaint::Color32::BLACK)
                );
            }
        }
}

#[allow(dead_code)]
enum Shape {
    LINE(Line),
    CIRCLE(Circle),
}
