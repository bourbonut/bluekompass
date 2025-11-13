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
    ChangeMode(Mode),
}

#[derive(Default)]
struct App {
    sketch: Sketch,
}

#[derive(Debug, Clone)]
enum Mode {
    Pan,
    ThreePointsCircle,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::ThreePointsCircle
    }
}

// TODO: In order to avoid Clone traits everywhere (especially when doing `sketch.clone()`), App
// structure should keep all important information and `Sketch` should only receive a `Vec<Shape>`.

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::PendingPoint(point) => self.sketch.add_point(point),
            Message::ChangeMode(mode) => self.sketch.change_mode(mode),
        };
    }

    fn view(&self) -> Element<'_, Message> {
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
