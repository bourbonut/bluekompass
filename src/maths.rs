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

pub fn compute_circle_center(a: &Point, b: &Point, c: &Point) -> Point {
    let a = point_to_vec2(a);
    let b = point_to_vec2(b);
    let c = point_to_vec2(c);
    let ab = b - a;
    let bc = c - b;
    let middle_ab = 0.5 * (a + b);
    let middle_bc = 0.5 * (b + c);
    if let Some(center) =
        compute_intersection_line_to_line(&middle_ab, &rot90(ab), &middle_bc, &rot90(bc))
    {
        Point::new(center.x, center.y)
    } else {
        Point::new(f32::NAN, f32::NAN)
    }
}

pub fn compute_circle_radius(center: &Point, circle_point: &Point) -> f32 {
    let center = point_to_vec2(center);
    let circle_point = point_to_vec2(circle_point);
    (center - circle_point).length()
}
