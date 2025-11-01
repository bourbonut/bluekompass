use iced::{
    advanced::mouse::ScrollDelta, event, mouse::Event as MouseEvent, widget::image::viewer,
    Element, Event, Length::Fill, Point, Subscription,
};

struct App {
    current_pos: Point<f32>,
    k: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_pos: Point::default(),
            k: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    EventOccured(Event),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::EventOccured(event) => match event {
                Event::Mouse(MouseEvent::WheelScrolled {
                    delta: ScrollDelta::Lines { x: 0.0, y },
                }) => {
                    if y > 0.0 {
                        self.k *= 2.0;
                    } else {
                        self.k *= 0.5;
                    }
                }
                Event::Mouse(MouseEvent::CursorMoved { position }) => {
                    self.current_pos = position;
                }
                _ => {}
            },
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccured)
    }

    fn view(&self) -> Element<'_, Message> {
        viewer("./assets/front.png".into())
            .width(Fill)
            .height(Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application("Viewer", App::update, App::view)
        .subscription(App::subscription)
        .run()
}
