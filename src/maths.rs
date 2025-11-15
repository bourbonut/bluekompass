use glam::Vec2;
use iced::Point;

fn point_to_vec2(v: &Point) -> Vec2 {
    Vec2::new(v.x, v.y)
}

fn rot90(v: Vec2) -> Vec2 {
    Vec2::new(v.y, -v.x)
}

fn compute_intersection_line_to_line(p1: &Vec2, d1: &Vec2, p2: &Vec2, d2: &Vec2) -> Option<Vec2> {
    let p1p2 = *p2 - *p1;
    let (a, b) = (*d1).into();
    let (c, d) = (*d2).into();
    let det = a * d - b * c;
    if det == 0. {
        return None;
    }
    let k = Vec2::new(d / det, -c / det).dot(p1p2);
    Some(k * *d1 + *p1)
}

fn compute_circle_center(a: &Vec2, b: &Vec2, c: &Vec2) -> Vec2 {
    let ab = b - a;
    let bc = c - b;
    let middle_ab = 0.5 * (a + b);
    let middle_bc = 0.5 * (b + c);
    if let Some(center) =
        compute_intersection_line_to_line(&middle_ab, &rot90(ab), &middle_bc, &rot90(bc))
    {
        center
    } else {
        Vec2::NAN
    }
}

fn compute_circle_radius(center: &Vec2, circle_point: &Vec2) -> f32 {
    (center - circle_point).length() as f32
}

pub fn circle(a: &Point, b: &Point, c: &Point) -> (Point, f32) {
    let a = point_to_vec2(a);
    let b = point_to_vec2(b);
    let c = point_to_vec2(c);
    let center = compute_circle_center(&a, &b, &c);
    let radius = compute_circle_radius(&center, &a);
    (Point::new(center.x, center.y), radius)
}
