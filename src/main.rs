use std::{fs, path::PathBuf, str::FromStr};

use iced::advanced::svg::Handle;
use iced::widget::{canvas, Column, Scrollable};
use iced::window::icon::from_file;
use iced::{
    widget::{svg::Svg, text, Button, Container, Row, Stack},
    Element, Length, Theme,
};
use rfd::FileDialog;

mod circle;
mod color;
mod style;
mod viewer;

use circle::Circle;
use color::Hex;
use style::{container_theme_filled, container_transparent, svg_tool_style, theme_style};
use viewer::Viewer;

#[derive(Debug, Clone)]
enum Message {
    // EventOccured(Event),
    Button,
    OpenFileDialog,
    ChangeTheme,
    SelectTheme(usize),
}

#[derive(Debug)]
enum Status {
    MainLayout,
    ChangeThemeLayout,
}

struct Icon(String, Message);

struct App {
    theme: Theme,
    icons: [Icon; 5],
    status: Status,
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
                Icon(
                    load_icon("./assets/folder-open.svg"),
                    Message::OpenFileDialog,
                ),
                Icon(load_icon("./assets/directional.svg"), Message::Button),
                Icon(load_icon("./assets/circle.svg"), Message::Button),
                Icon(load_icon("./assets/spline.svg"), Message::Button),
                Icon(load_icon("./assets/palette.svg"), Message::ChangeTheme),
            ],
            status: Status::MainLayout,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::OpenFileDialog => {
                let file = FileDialog::new()
                    .add_filter("text", &["txt", "rs"])
                    .add_filter("rust", &["rs", "toml"])
                    .set_directory("/")
                    .pick_file();
                println!("Open File Dialog: {:?}", file);
            }
            Message::Button => {
                println!("Button pressed");
            }
            Message::SelectTheme(idx) => {
                self.theme = Theme::ALL.get(idx).unwrap().clone();
                println!("Selected theme: {:?}", self.theme);
            }
            Message::ChangeTheme => {
                match self.status {
                    Status::ChangeThemeLayout => {
                        self.status = Status::MainLayout;
                    }
                    _ => {
                        self.status = Status::ChangeThemeLayout;
                    }
                };
                println!("Current status: {:?}", self.status);
            }
        }
    }

    // fn subscription(&self) -> Subscription<Message> {
    //     event::listen().map(Message::EventOccured)
    // }

    fn view(&self) -> Element<'_, Message> {
        Stack::with_children(match self.status {
            Status::MainLayout => vec![self.viewer(), self.tools()],
            Status::ChangeThemeLayout => vec![self.viewer(), self.tools(), self.available_theme()],
        })
        .width(Length::Shrink)
        .into()
    }

    /// Creates a viewer where the image is drawn
    fn viewer(&self) -> Element<'_, Message> {
        Viewer::new("./assets/front.png")
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// Creates a `Container` with all tool buttons distributed into one row
    fn tools(&self) -> Element<'_, Message> {
        Container::new(
            Row::from_vec((0..self.icons.len()).map(|i| self.svg_button(i)).collect()).spacing(2.5),
        )
        .style(container_transparent)
        .width(Length::Shrink)
        .padding(10.)
        .center_x(Length::Fill)
        .into()
    }

    /// Creates a `Container` with all available themes distributed into one column and into a
    /// `Scrollable` widget
    fn available_theme(&self) -> Element<'_, Message> {
        let themes = Theme::ALL
            .iter()
            .enumerate()
            .map(|(i, theme)| {
                Button::new(Row::from_vec(vec![
                    canvas(Circle {
                        radius: 5.,
                        border_radius: 2.,
                        fill_color: theme.palette().primary,
                        border_color: theme.palette().text,
                    })
                    .width(20.)
                    .height(20.)
                    .into(),
                    text(format!("{:?}", theme)).into(),
                ]))
                .style(theme_style)
                .width(200.)
                .on_press(Message::SelectTheme(i))
                .into()
            })
            .collect();
        Container::new(
            Container::new(Column::from_vec(vec![
                text("Available themes").size(14).into(),
                Scrollable::new(Column::from_vec(themes).spacing(5.)).into(),
            ]))
            .height(Length::Fill)
            .style(container_theme_filled),
        )
        .align_right(Length::Fill)
        .into()
    }

    /// Creates a button containing an SVG icon, with the current theme applied to it.
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
                .width(26.)
                .height(26.),
        )
        .padding(2.)
        .on_press(icon_message.clone())
        .style(svg_tool_style)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application("Bluekompass", App::update, App::view)
        .theme(|state| state.theme.clone()) // Allow to change the theme
        .window(iced::window::Settings {
            // Add window icon
            icon: Some(
                from_file("./assets/bluekompass.png")
                    .expect("Cannot find the icon in 'assets' folder"),
            ),
            ..Default::default()
        })
        // .subscription(App::subscription) // For events (mouse, keyboard, ...)
        .run()
}
