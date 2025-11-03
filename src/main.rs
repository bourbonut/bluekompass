use iced::{
    theme::Palette,
    widget::{button, container, row, svg, Button, Container, Stack},
    Background, Border, Element, Length, Shadow, Theme,
};

mod viewer;
use viewer::Viewer;

#[derive(Default, Debug)]
struct App {
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    // EventOccured(Event),
    Button,
}

fn styled(palette: Palette) -> button::Style {
    button::Style {
        background: Some(Background::Color(palette.background)),
        text_color: palette.text,
        shadow: Shadow::default(),
        border: Border {
            color: palette.background,
            width: 1.0,
            radius: 10.0.into(),
        },
    }
}

fn svg_button(path: &str) -> Element<'_, Message> {
    Button::new(svg(path).width(Length::Shrink).height(Length::Shrink))
        .padding(2.)
        .on_press(Message::Button)
        .style(|theme: &Theme, status: button::Status| {
            let base = styled(theme.palette());
            match status {
                button::Status::Hovered => button::Style {
                    background: Some(Background::Color(theme.palette().primary.scale_alpha(0.5))),
                    ..base
                },
                _ => base,
            }
        })
        .into()
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Button => {
                println!("Button pressed");
            }
        }
    }

    // fn subscription(&self) -> Subscription<Message> {
    //     event::listen().map(Message::EventOccured)
    // }

    fn view(&self) -> Element<'_, Message> {
        Stack::with_children([
            Viewer::new("./assets/front.png")
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            Container::new(
                row![
                    svg_button("./assets/directional.svg"),
                    svg_button("./assets/circle.svg"),
                ]
                .spacing(2.5),
            )
            .style(|_: &Theme| container::Style {
                background: Some(Background::Color(iced::Color::TRANSPARENT)),
                ..Default::default()
            })
            .width(Length::Fill)
            .padding(10.)
            .center_x(Length::Fill)
            .into(),
        ])
        .into()
    }
}

fn main() -> iced::Result {
    iced::application("Viewer", App::update, App::view)
        .theme(|state| {
            // Allow to change the theme
            state.theme.clone()
        })
        // .subscription(App::subscription) // For events
        .run()
}
