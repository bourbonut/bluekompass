use iced::Background;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::widget::Canvas;
use iced::widget::Container;

mod sketch;
use sketch::Sketch;

#[derive(Debug)]
enum Message {}

#[derive(Default)]
struct App {}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let sketch = Sketch::default();
        Container::new(
            Container::new(Canvas::new(sketch).width(Length::Fill).height(Length::Fill)).style(
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
        .run()
}
