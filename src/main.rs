use std::{fs, path::PathBuf, str::FromStr};

use iced::advanced::svg::Handle;
use iced::window::icon::from_file;
use iced::{
    theme::Palette,
    widget::{button, container, svg::Svg, Button, Container, Row, Stack},
    Background, Border, Element, Length, Shadow, Theme,
};
use rfd::FileDialog;

mod color;
mod viewer;

use color::Hex;
use viewer::Viewer;

#[derive(Debug, Clone)]
enum Message {
    // EventOccured(Event),
    Button,
    OpenFileDialog,
}

#[derive(Debug)]
struct Icon(String, Message);

#[derive(Debug)]
struct App {
    theme: Theme,
    icons: [Icon; 5],
}

fn load_icon(file_path: &str) -> String {
    let path =
        PathBuf::from_str(file_path).expect(format!("'{}' should exists.", file_path).as_str());
    fs::read_to_string(path)
        .expect(format!("It should have been able to read '{}'", file_path).as_str())
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

impl Default for App {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            icons: [
                Icon(
                    load_icon("./assets/folder-open.svg"),
                    Message::OpenFileDialog,
                ),
                Icon(load_icon("./assets/directional.svg"), Message::Button),
                Icon(load_icon("./assets/circle.svg"), Message::Button),
                Icon(load_icon("./assets/spline.svg"), Message::Button),
                Icon(load_icon("./assets/palette.svg"), Message::Button),
            ],
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Button => {
                println!("Button pressed");
            }
            Message::OpenFileDialog => {
                let file = FileDialog::new()
                    .add_filter("text", &["txt", "rs"])
                    .add_filter("rust", &["rs", "toml"])
                    .set_directory("/")
                    .pick_file();
                println!("Open File Dialog: {:?}", file);
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
                Row::from_vec((0..self.icons.len()).map(|i| self.svg_button(i)).collect())
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

    fn svg_button(&self, icon_idx: usize) -> Element<'_, Message> {
        let primary = self.theme.palette().primary.into_hex();
        let text = self.theme.palette().text.into_hex();
        let Icon(icon_string, icon_message) = &self.icons[icon_idx];
        let modified_icon = icon_string
            .replace("currentColor", &primary)
            .replace("white", &text)
            .into_bytes();
        Button::new(
            Svg::new(Handle::from_memory(modified_icon))
                .width(Length::Shrink)
                .height(Length::Shrink),
        )
        .padding(2.)
        .on_press(icon_message.clone())
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
}

fn main() -> iced::Result {
    iced::application("Bluekompass", App::update, App::view)
        .theme(|state| {
            // Allow to change the theme
            state.theme.clone()
        })
        .window(iced::window::Settings {
            icon: Some(
                from_file("./assets/bluekompass.png")
                    .expect("Cannot find the icon in 'assets' folder"),
            ),
            ..Default::default()
        })
        // .subscription(App::subscription) // For events
        .run()
}
