use std::{fs, path::PathBuf, str::FromStr};

use iced::advanced::svg::Handle;
use iced::{
    theme::Palette,
    widget::{button, container, svg::Svg, Button, Container, Row, Stack},
    Background, Border, Element, Length, Shadow, Theme,
};
use rfd::FileDialog;

mod viewer;
use viewer::Viewer;

#[derive(Debug)]
struct App {
    theme: Theme,
    icons: [String; 3],
}

fn load_icon(file_path: &str) -> String {
    let path =
        PathBuf::from_str(file_path).expect(format!("'{}' should exists.", file_path).as_str());
    fs::read_to_string(path)
        .expect(format!("It should have been able to read '{}'", file_path).as_str())
}

impl Default for App {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            icons: [
                load_icon("./assets/folder-open.svg"),
                load_icon("./assets/directional.svg"),
                load_icon("./assets/circle.svg"),
            ],
        }
    }
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

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Button => {
                let files = FileDialog::new()
                    .add_filter("text", &["txt", "rs"])
                    .add_filter("rust", &["rs", "toml"])
                    .set_directory("/")
                    .pick_file();
                println!("Button pressed: {:?}", files);
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
        let modified_icon = self.icons[icon_idx]
            .replace("currentColor", &primary)
            .replace("white", &text)
            .into_bytes();
        Button::new(
            Svg::new(Handle::from_memory(modified_icon))
                .width(Length::Shrink)
                .height(Length::Shrink),
        )
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
}

fn color_f32_to_u8(pigment: &f32) -> u8 {
    (pigment * 255.) as u8
}

trait Hex {
    fn into_hex(&self) -> String;
}

impl Hex for iced::Color {
    fn into_hex(&self) -> String {
        format!(
            "#{:x}{:x}{:x}",
            color_f32_to_u8(&self.r),
            color_f32_to_u8(&self.g),
            color_f32_to_u8(&self.b)
        )
    }
}

fn main() -> iced::Result {
    let path = PathBuf::from_str("./assets/folder-open.svg")
        .expect("'assets/folder-open.svg' should exists.");
    let color = Theme::Dracula.palette().text;
    let content = fs::read_to_string(path)
        .expect("It should have been able to read 'assets/folder-open.svg'")
        .replace("currentColor", color.into_hex().as_str());
    println!("{content:?}");
    iced::application("Bluekompass", App::update, App::view)
        .theme(|state| {
            // Allow to change the theme
            state.theme.clone()
        })
        // .subscription(App::subscription) // For events
        .run()
}
