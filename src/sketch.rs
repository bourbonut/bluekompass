use crate::shapes;
use crate::Message;
use crate::Mode;
use iced::advanced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::Stroke;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Theme;
use iced::Vector;

pub struct Sketch<'a> {
    shapes: &'a Vec<shapes::Shape>,
    points: &'a Vec<Point>,
    mode: &'a Mode,
    min_scale: f32,
    max_scale: f32,
}

impl<'a> Sketch<'a> {
    pub fn new(shapes: &'a Vec<shapes::Shape>, points: &'a Vec<Point>, mode: &'a Mode) -> Self {
        Self {
            shapes,
            points,
            mode,
            min_scale: 0.25,
            max_scale: 10.0,
        }
    }
}

pub struct State {
    current_offset: Vector<f32>,
    starting_offset: Vector<f32>,
    cursor_grabbed_at: Option<Vector<f32>>,
    scale: f32,
    hovered: Option<usize>,
    previous_mode: Mode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_offset: Vector::new(0., 0.),
            starting_offset: Vector::new(0., 0.),
            cursor_grabbed_at: None,
            scale: 1.0,
            hovered: None,
            previous_mode: Mode::Selection,
        }
    }
}

impl<'a> canvas::Program<Message> for Sketch<'a> {
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
        let hovered_index = state.hovered;

        for (i, shape) in self.shapes.iter().enumerate() {
            match shape {
                shapes::Shape::Circle { center, radius } => {
                    let circle_position = Point::new(
                        center.x * state.scale + state.current_offset.x,
                        center.y * state.scale + state.current_offset.y,
                    );
                    let circle = canvas::Path::circle(circle_position, radius * state.scale);

                    frame.stroke(
                        &circle,
                        Stroke::default()
                            .with_color(if Some(i) == hovered_index {
                                theme.palette().primary
                            } else {
                                theme.palette().background
                            })
                            .with_width(shapes::BORDER_RADIUS),
                    );
                }
            }
        }

        for point in self.points.iter() {
            let circle_position = Point::new(
                point.x * state.scale + state.current_offset.x,
                point.y * state.scale + state.current_offset.y,
            );
            let filled_circle = canvas::Path::circle(circle_position, shapes::POINT_RADIUS);
            let border_circle = canvas::Path::circle(
                circle_position,
                shapes::POINT_RADIUS + shapes::BORDER_RADIUS,
            );

            // And fill it with some color
            frame.fill(&border_circle, theme.palette().background);
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

                    if y < 0.0 && previous_scale > self.min_scale
                        || y > 0.0 && previous_scale < self.max_scale
                    {
                        state.scale = if y > 0.0 {
                            state.scale * (1.0 + 0.1)
                        } else {
                            state.scale / (1.0 + 0.1)
                        }
                        .clamp(self.min_scale, self.max_scale);

                        let factor = state.scale / previous_scale;

                        state.current_offset =
                            state.current_offset * factor + cursor_position * (1. - factor);
                    }
                }
                canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Middle)) => {
                    if cursor.position_over(bounds).is_some() {
                        state.cursor_grabbed_at = Some(cursor_position);
                        state.starting_offset = state.current_offset;
                        state.previous_mode = self.mode.clone();
                        message = Some(Message::ChangeMode(Mode::Selection));
                    }
                }
                canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Middle)) => {
                    state.cursor_grabbed_at = None;
                    message = Some(Message::ChangeMode(state.previous_mode.clone()));
                }
                canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    if cursor.position_over(bounds).is_some() {
                        match self.mode {
                            Mode::Selection => {
                                state.cursor_grabbed_at = Some(cursor_position);
                                state.starting_offset = state.current_offset;
                            }
                            Mode::Circle => {
                                status = canvas::event::Status::Captured;
                                message = Some(Message::PendingPoint(Point::new(
                                    (position.x - state.current_offset.x) / state.scale,
                                    (position.y - state.current_offset.y) / state.scale,
                                )));
                            }
                        }
                    }
                }
                canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    state.cursor_grabbed_at = None;
                }
                canvas::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    if cursor.position_over(bounds).is_some() {
                        if let Some(origin) = state.cursor_grabbed_at {
                            let delta = origin - Vector::new(position.x, position.y);

                            state.current_offset = Vector::new(
                                state.starting_offset.x - delta.x,
                                state.starting_offset.y - delta.y,
                            );
                        }
                    } else {
                        state.cursor_grabbed_at = None;
                    }
                }
                _ => (),
            }
            for (idx, shape) in self.shapes.iter().enumerate() {
                if shape.is_hovered(&position, &state.current_offset, &state.scale) {
                    state.hovered = Some(idx);
                } else {
                    state.hovered = None;
                }
            }
        }
        (status, message)
    }
}
