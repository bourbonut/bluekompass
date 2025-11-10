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

fn length_squared(v: Vector<f32>) -> f32 {
    v.x * v.x + v.y * v.y
}

fn is_inside_circle(cursor: Point<f32>, center: Point<f32>, radius: f32) -> bool {
    let radius2 = radius * radius;
    (length_squared(cursor - center).abs() - radius2) <= 0.
}

impl Circle {
    pub fn total_radius(&self, scale: &f32) -> f32 {
        return self.border_radius + self.radius * scale;
    }
}

struct State {
    fill_color: Option<Color>,
    current_offset: Vector<f32>,
    starting_offset: Vector<f32>,
    cursor_grabbed_at: Option<Vector<f32>>,
    scale: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_offset: Vector::new(0., 0.),
            starting_offset: Vector::new(0., 0.),
            cursor_grabbed_at: None,
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
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // We create a `Path` representing a simple circle
        let circle_position = Point::new(
            self.position.x + state.current_offset.x,
            self.position.y + state.current_offset.y,
        );
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
        _bounds: Rectangle,
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

                        let adjustment =
                            (self.position + state.current_offset - cursor_position) * factor;

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
                if is_inside_circle(
                    position,
                    Point::new(
                        self.position.x + state.current_offset.x,
                        self.position.y + state.current_offset.y,
                    ),
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
            position: Vector::new(100., 100.),
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
