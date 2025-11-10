use std::f32::NAN;

use iced::Point;
use iced::Rectangle;
use iced::Vector;

enum Normalizer {
    Constant(f32),
    Linear { a: f32, b: f32 },
}

impl Normalizer {
    fn apply(&self, x: f32) -> f32 {
        match self {
            Normalizer::Constant(v) => *v,
            Normalizer::Linear { a, b } => (x - *a) / *b,
        }
    }
}

fn normalize(a: f32, b: f32) -> Normalizer {
    let b = b - a;

    if b.is_nan() {
        Normalizer::Constant(NAN)
    } else if b == 0. {
        Normalizer::Constant(0.5)
    } else {
        Normalizer::Linear { a, b }
    }
}

struct Interpolator(f32, f32);

impl Interpolator {
    fn apply(&self, t: f32) -> f32 {
        self.0 * (1. - t) + self.1 * t
    }
}

fn interpolate(a: f32, b: f32) -> Interpolator {
    Interpolator(a, b)
}

struct BiMap {
    d0: Normalizer,
    r0: Interpolator,
}

impl BiMap {
    fn new(domain: &[f32; 2], range: &[f32; 2]) -> Self {
        let d0 = domain[0];
        let d1 = domain[1];
        let r0 = range[0];
        let r1 = range[1];
        if d1 < d0 {
            Self {
                d0: normalize(d1, d0),
                r0: interpolate(r1, r0),
            }
        } else {
            Self {
                d0: normalize(d0, d1),
                r0: interpolate(r0, r1),
            }
        }
    }

    fn apply(&self, x: f32) -> f32 {
        self.r0.apply(self.d0.apply(x))
    }
}

pub struct LinearScaler {
    input: BiMap,
    output: BiMap,
}

impl LinearScaler {
    pub fn new(domain: &[f32; 2], range: &[f32; 2]) -> Self {
        Self {
            input: BiMap::new(range, domain),
            output: BiMap::new(domain, range),
        }
    }

    pub fn apply(&self, x: f32) -> f32 {
        self.output.apply(x)
    }

    pub fn invert(&self, x: f32) -> f32 {
        self.input.apply(x)
    }
}

pub struct LinearScaler2D {
    pub x_scaler: LinearScaler,
    pub y_scaler: LinearScaler,
}

impl LinearScaler2D {
    pub fn new(bounds: Rectangle) -> Self {
        Self {
            x_scaler: LinearScaler::new(&[0., 1.], &[bounds.x, bounds.width]),
            y_scaler: LinearScaler::new(&[0., 1.], &[bounds.y, bounds.height]),
        }
    }
    pub fn apply(&self, v: Vector<f32>) -> Point<f32> {
        Point::new(self.x_scaler.apply(v.x), self.y_scaler.apply(v.y))
    }

    pub fn invert(&self, p: Point<f32>) -> Vector<f32> {
        Vector::new(self.x_scaler.invert(p.x), self.y_scaler.invert(p.y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize(0., 0.).apply(10.), 0.5);
        assert_eq!(normalize(1., 1.).apply(10.), 0.5);
        assert!(normalize(1., NAN).apply(10.).is_nan());
        assert_eq!(normalize(1., 2.).apply(10.), (10. - 1.) / (2. - 1.));
    }

    #[test]
    fn test_interpolate() {
        assert_eq!(interpolate(10., 20.).apply(0.5), 15.);
        assert_eq!(interpolate(10., 20.).apply(0.), 10.);
        assert_eq!(interpolate(10., 20.).apply(1.), 20.);
        assert_eq!(interpolate(10., 25.).apply(1.), 25.);
    }
    #[test]
    fn test_linear_scaler() {
        let scaler = LinearScaler::new(&[-250., 250.], &[0., 1080.]);
        assert_eq!(scaler.apply(0.), 1080. / 2.);
        assert_eq!(scaler.invert(1080. / 2.), 0.);

        assert_eq!(scaler.apply(-250.), 0.);
        assert_eq!(scaler.invert(0.), -250.);

        assert_eq!(scaler.apply(250.), 1080.);
        assert_eq!(scaler.invert(1080.), 250.);
    }
}
