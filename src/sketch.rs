use crate::Message;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Theme;
use iced::Vector;
use iced::advanced::mouse;
use iced::widget::canvas;

enum Shape {
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
                let radius = radius * scale + 2.;
                let radius2 = radius * radius;
                (length_squared(*cursor - center).abs() - radius2) <= 0.
            }
        }
    }
}

enum Pending {
    CircleOnePoint(usize),
    CircleTwoPoints(usize, usize),
}

#[derive(Default)]
pub struct Sketch {
    shapes: Vec<Shape>,
    pending: Vec<Pending>,
    points: Vec<Point>,
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
                        canvas::Path::circle(circle_position, radius * state.scale + 2.);

                    // And fill it with some color
                    frame.fill(&border_circle, theme.palette().text);
                    frame.fill(&filled_circle, theme.palette().primary);
                }
            }
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
        match cursor.position() {
            Some(position) => {
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
                        state.cursor_grabbed_at = Some(cursor_position);
                        state.starting_offset = state.current_offset;
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
            None => (),
        }
        (canvas::event::Status::Ignored, None)
    }
}
