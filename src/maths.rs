use core::f32;

use egui_plot::PlotPoint;
use eframe::egui::Vec2;

fn compute_intersection_line_to_line(p1: &Vec2, d1: &Vec2, p2: &Vec2, d2: &Vec2) -> Option<Vec2> {
    let p1p2 = *p2 - *p1;
    let (a, b) = d1.into();
    let (c, d) = d2.into();
    let det = a * d - b * c;
    if det == 0. { return None; }
    let k = Vec2::new(d / det, -c / det).dot(p1p2);
    Some(k * *d1 + *p1)
}

pub fn compute_circle_center(points: &[PlotPoint; 3]) -> PlotPoint {
    let [a, b, c] = points.map(|p| p.to_vec2());
    let ab = b - a;
    let bc = c - b;
    let middle_ab = 0.5 * (a + b);
    let middle_bc = 0.5 * (b + c);
    if let Some(center) = compute_intersection_line_to_line(&middle_ab, &ab.rot90(), &middle_bc, &bc.rot90()) {
        PlotPoint::new(center.x, center.y)
    } else {
        PlotPoint::new(f32::NAN, f32::NAN)
    }
}

pub fn compute_circle_radius(center: &Vec2, circle_point: &Vec2) -> f32 {
    (*center - *circle_point).length()
}
