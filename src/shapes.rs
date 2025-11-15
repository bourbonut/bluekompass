use crate::maths;
use glam::usize;
use iced::Point;
use iced::Vector;

pub static BORDER_RADIUS: f32 = 2.;
pub static POINT_RADIUS: f32 = 7.;

#[allow(dead_code)]
pub enum Primitive {
    Circle { points: [usize; 3] },
}

#[derive(Debug)]
pub enum Shape {
    Circle { center: Point, radius: f32 },
}

fn length_squared(v: Vector) -> f32 {
    v.x * v.x + v.y * v.y
}

impl Shape {
    pub fn is_inside(&self, cursor: &Point, offset: &Vector, scale: &f32) -> bool {
        match self {
            Self::Circle { center, radius } => {
                let center = Point::new(center.x + offset.x, center.y + offset.y);
                let radius = radius * scale + BORDER_RADIUS;
                let radius2 = radius * radius;
                (length_squared(*cursor - center).abs() - radius2) <= 0.
            }
        }
    }
}

impl Shape {
    pub fn circle(a: &Point, b: &Point, c: &Point) -> Shape {
        let (center, radius) = maths::circle(a, b, c);
        Shape::Circle { center, radius }
    }
}
