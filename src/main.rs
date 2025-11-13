use iced::Element;
use iced::Length;
use iced::Point;
use iced::widget::Canvas;
use iced::widget::Container;

mod sketch;
use sketch::Sketch;

#[derive(Debug)]
enum Message {
    PendingPoint(Point),
}

#[derive(Default)]
struct App {
    sketch: Sketch,
}

#[derive(Clone)]
enum Mode {
    Pan,
    ThreePointsCircle,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::ThreePointsCircle
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::PendingPoint(point) => self.sketch.add_point(point),
        };
    }

    fn view(&self) -> Element<'_, Message> {
        // sketch.shapes.push(sketch::Shape::Circle {
        //     center: iced::Point::new(100., 100.),
        //     radius: 10.,
        //     points: [0; 3],
        // });
        Container::new(Container::new(
            Canvas::new(self.sketch.clone())
                .width(Length::Fill)
                .height(Length::Fill),
        ))
        .center(Length::Fill)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application("Bluekompass", App::update, App::view)
        .antialiasing(true)
        .run()
}
