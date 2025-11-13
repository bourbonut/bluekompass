use crate::Message;
use crate::Mode;
use glam::Vec2;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Theme;
use iced::Vector;
use iced::advanced::mouse;
use iced::widget::canvas;

static BORDER_RADIUS: f32 = 2.;
static POINT_RADIUS: f32 = 7.;

#[derive(Clone, Debug)]
pub enum Shape {
    Circle {
        center: Point<f32>,
        radius: f32,
        points: [usize; 3],
    },
}

fn length_squared(v: Vector<f32>) -> f32 {
    v.x * v.x + v.y * v.y
}

impl Shape {
    fn is_inside(&self, cursor: &Point<f32>, offset: &Vector<f32>, scale: &f32) -> bool {
        match self {
            Self::Circle {
                center,
                radius,
                points: _,
            } => {
                let center = Point::new(center.x + offset.x, center.y + offset.y);
                let radius = radius * scale + BORDER_RADIUS;
                let radius2 = radius * radius;
                (length_squared(*cursor - center).abs() - radius2) <= 0.
            }
        }
    }
}

#[derive(Clone)]
enum Pending {
    CircleOnePoint(usize),
    CircleTwoPoints(usize, usize),
    None,
}

impl Default for Pending {
    fn default() -> Self {
        Pending::None
    }
}

#[derive(Clone, Default)]
pub struct Sketch {
    pub shapes: Vec<Shape>,
    pub pending: Pending,
    pub points: Vec<Point>,
    pub mode: Mode,
}

pub struct State {
    current_offset: Vector<f32>,
    starting_offset: Vector<f32>,
    cursor_grabbed_at: Option<Vector<f32>>,
    scale: f32,
    hovered: Option<usize>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_offset: Vector::new(0., 0.),
            starting_offset: Vector::new(0., 0.),
            cursor_grabbed_at: None,
            scale: 1.0,
            hovered: None,
        }
    }
}

// Then, we implement the `Program` trait
impl canvas::Program<Message> for Sketch {
    // No internal state
    type State = State;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        for shape in self.shapes.iter() {
            match shape {
                Shape::Circle {
                    center,
                    radius,
                    points: _,
                } => {
                    let circle_position = Point::new(
                        center.x + state.current_offset.x,
                        center.y + state.current_offset.y,
                    );
                    let filled_circle = canvas::Path::circle(circle_position, radius * state.scale);
                    let border_circle =
                        canvas::Path::circle(circle_position, radius * state.scale + BORDER_RADIUS);

                    // And fill it with some color
                    frame.fill(&border_circle, theme.palette().text);
                    frame.fill(&filled_circle, theme.palette().primary);
                }
            }
        }

        for point in self.points.iter() {
            let circle_position = Point::new(
                point.x + state.current_offset.x,
                point.y + state.current_offset.y,
            );
            let filled_circle = canvas::Path::circle(circle_position, POINT_RADIUS * state.scale);
            let border_circle =
                canvas::Path::circle(circle_position, POINT_RADIUS * state.scale + BORDER_RADIUS);

            // And fill it with some color
            frame.fill(&border_circle, theme.palette().text);
            frame.fill(&filled_circle, theme.palette().primary);
        }

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        let mut message = None;
        let mut status = canvas::event::Status::Ignored;
        if let Some(position) = cursor.position() {
            let cursor_position = Vector::new(position.x, position.y);
            match event {
                canvas::Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                    let (mouse::ScrollDelta::Lines { y, .. }
                    | mouse::ScrollDelta::Pixels { y, .. }) = delta;

                    let previous_scale = state.scale;
                    state.scale = if y > 0.0 {
                        state.scale * (1.0 + 0.1)
                    } else {
                        state.scale / (1.0 + 0.1)
                    };

                    let factor = state.scale / previous_scale - 1.0;

                    let center = {
                        let center = bounds.center();
                        Vector::new(center.x, center.y)
                    };

                    let adjustment = (center + state.current_offset - cursor_position) * factor;

                    state.current_offset = state.current_offset + adjustment;
                }
                canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    match self.mode {
                        Mode::Pan => {
                            state.cursor_grabbed_at = Some(cursor_position);
                            state.starting_offset = state.current_offset;
                        }
                        Mode::ThreePointsCircle => {
                            status = canvas::event::Status::Captured;
                            message = Some(Message::PendingPoint(position));
                        }
                    }
                }
                canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    state.cursor_grabbed_at = None;
                }
                canvas::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    if let Some(origin) = state.cursor_grabbed_at {
                        let delta = origin - Vector::new(position.x, position.y);

                        state.current_offset = Vector::new(
                            state.starting_offset.x - delta.x,
                            state.starting_offset.y - delta.y,
                        );
                    }
                }
                _ => (),
            }
            for (idx, shape) in self.shapes.iter().enumerate() {
                if shape.is_inside(&position, &state.current_offset, &state.scale) {
                    state.hovered = Some(idx);
                } else {
                    state.hovered = None;
                }
            }
        }
        (status, message)
    }
}

impl Sketch {
    pub fn add_point(&mut self, point: Point) {
        match self.pending {
            Pending::None => {
                self.pending = Pending::CircleOnePoint(self.points.len());
                self.points.push(point);
                println!("1 - Pending point {}", point);
            }
            Pending::CircleOnePoint(i1) => {
                self.pending = Pending::CircleTwoPoints(i1, self.points.len());
                self.points.push(point);
                println!("2 - Pending point {}", point);
            }
            Pending::CircleTwoPoints(i1, i2) => {
                self.pending = Pending::None;
                let center = compute_circle_center(&point, &self.points[i1], &self.points[i2]);
                let i3 = self.points.len();
                self.points.push(point);
                let radius = compute_circle_radius(&point_to_vec2(&center), &point_to_vec2(&point));
                let points = [i1, i2, i3];
                self.shapes.push(Shape::Circle {
                    center,
                    radius,
                    points,
                });
                println!("3 - Shape {:?}", self.shapes[0]);
            }
        }
    }
}

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

fn compute_circle_center(a: &Point, b: &Point, c: &Point) -> Point {
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

pub fn compute_circle_radius(center: &Vec2, circle_point: &Vec2) -> f32 {
    (*center - *circle_point).length()
}
