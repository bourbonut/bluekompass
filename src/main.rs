use glam::Vec2;
use iced::Background;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Theme;
use iced::advanced::mouse;
use iced::widget::Canvas;
use iced::widget::Container;
use iced::widget::canvas;

trait IntoVec2 {
    fn into(self) -> Vec2;
}

impl IntoVec2 for Point {
    fn into(self) -> Vec2 {
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
    pub radius: f32,
    pub border_radius: f32,
    pub fill_color: Color,
    pub border_color: Color,
}

fn is_inside_circle(cursor: impl IntoVec2, center: impl IntoVec2, radius: f32) -> bool {
    let radius2 = radius * radius;
    ((cursor.into() - center.into()).length_squared().abs() - radius2) <= 0.
}

impl Circle {
    pub fn total_radius(&self) -> f32 {
        return self.border_radius + self.radius;
    }
}

// Then, we implement the `Program` trait
impl canvas::Program<Message> for Circle {
    // No internal state
    type State = Option<Color>;

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
        let filled_circle = canvas::Path::circle(frame.center(), self.radius);
        let border_circle = canvas::Path::circle(frame.center(), self.radius + self.border_radius);

        // And fill it with some color
        frame.fill(&border_circle, self.border_color);
        frame.fill(
            &filled_circle,
            match *state {
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
        _event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        match cursor.position() {
            Some(position) => {
                if is_inside_circle(position, bounds.center(), self.total_radius()) {
                    *state = Some(self.fill_color.scale_alpha(0.5));
                    (
                        canvas::event::Status::Captured,
                        Some(Message::CursorMoved(position)),
                    )
                } else {
                    *state = Some(self.fill_color);
                    (canvas::event::Status::Ignored, None)
                }
            }
            None => {
                *state = Some(self.fill_color);
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
            Message::CursorMoved(point) => {
                println!("{:?}", point);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let circle = Circle {
            radius: 30.,
            border_radius: 3.,
            fill_color: Theme::Dark.palette().primary,
            border_color: Theme::Dark.palette().text,
        };
        let r = circle.total_radius() * 2.;
        Container::new(
            Container::new(Canvas::new(circle).width(r).height(r)).style(|_| {
                iced::widget::container::Style {
                    background: Some(Background::Color(Color::WHITE)),
                    ..Default::default()
                }
            }),
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
