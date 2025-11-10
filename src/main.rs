use glam::Vec2;
use iced::Background;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Theme;
use iced::Vector;
use iced::advanced::mouse;
use iced::widget::Canvas;
use iced::widget::Container;
use iced::widget::canvas;

mod linear_scaler;
use linear_scaler::LinearScaler2D;

trait IntoVec2 {
    fn into_vec2(self) -> Vec2;
}

impl IntoVec2 for Point {
    fn into_vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Debug, Clone)]
enum Message {
    CursorMoved(Point),
}

// *                 Circle shape                    *

#[derive(Debug)]
pub struct Circle {
    pub position: Vector<f32>,
    pub radius: f32,
    pub border_radius: f32,
    pub fill_color: Color,
    pub border_color: Color,
}

fn is_inside_circle(cursor: impl IntoVec2, center: impl IntoVec2, radius: f32) -> bool {
    let radius2 = radius * radius;
    ((cursor.into_vec2() - center.into_vec2())
        .length_squared()
        .abs()
        - radius2)
        <= 0.
}

impl Circle {
    pub fn total_radius(&self, scale: &f32) -> f32 {
        return self.border_radius + self.radius * scale;
    }
}

struct State {
    fill_color: Option<Color>,
    current_offset: Vector<f32>,
    scale: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_offset: Vector::new(0., 0.),
            fill_color: None,
            scale: 1.0,
        }
    }
}

// Then, we implement the `Program` trait
impl canvas::Program<Message> for Circle {
    // No internal state
    type State = State;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let scaler = LinearScaler2D::new(bounds);
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // We create a `Path` representing a simple circle
        let circle_position = scaler.apply(self.position + state.current_offset);
        let filled_circle = canvas::Path::circle(circle_position, self.radius * state.scale);
        let border_circle = canvas::Path::circle(
            circle_position,
            self.radius * state.scale + self.border_radius,
        );

        // And fill it with some color
        frame.fill(&border_circle, self.border_color);
        frame.fill(
            &filled_circle,
            match state.fill_color {
                Some(color) => color,
                None => self.fill_color,
            },
        );

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
                let scaler = LinearScaler2D::new(bounds);
                if let canvas::Event::Mouse(mouse::Event::WheelScrolled { delta }) = event {
                    let (mouse::ScrollDelta::Lines { y, .. }
                    | mouse::ScrollDelta::Pixels { y, .. }) = delta;
                    let cursor_position = scaler.invert(position);

                    let previous_scale = state.scale;
                    state.scale = if y > 0.0 {
                        state.scale * (1.0 + 0.1)
                    } else {
                        state.scale / (1.0 + 0.1)
                    };

                    let factor = state.scale / previous_scale - 1.0;

                    let adjustment =
                        (self.position + state.current_offset - cursor_position) * factor;

                    state.current_offset = state.current_offset + adjustment;
                }
                if is_inside_circle(
                    position,
                    scaler.apply(self.position + state.current_offset),
                    self.total_radius(&state.scale),
                ) {
                    state.fill_color = Some(self.fill_color.scale_alpha(0.5));
                    (
                        canvas::event::Status::Captured,
                        Some(Message::CursorMoved(position)),
                    )
                } else {
                    state.fill_color = Some(self.fill_color);
                    (canvas::event::Status::Ignored, None)
                }
            }
            None => {
                state.fill_color = Some(self.fill_color);
                (canvas::event::Status::Ignored, None)
            }
        }
    }
}

// *                 Application                     *

#[derive(Default)]
struct App {}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::CursorMoved(_point) => {
                // println!("{:?}", point);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let circle = Circle {
            position: Vector::new(0.5, 0.5),
            radius: 30.,
            border_radius: 3.,
            fill_color: Theme::Dark.palette().primary,
            border_color: Color {
                r: 1.,
                g: 0.,
                b: 0.,
                a: 1.,
            },
        };
        Container::new(
            Container::new(Canvas::new(circle).width(Length::Fill).height(Length::Fill)).style(
                |_| iced::widget::container::Style {
                    background: Some(Background::Color(Color::WHITE)),
                    ..Default::default()
                },
            ),
        )
        .center(Length::Fill)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application("Bluekompass", App::update, App::view)
        .antialiasing(true)
        // .subscription(App::subscription)
        .run()
}
