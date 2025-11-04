use std::{fs, path::PathBuf, str::FromStr};

use iced::advanced::svg::Handle;
use iced::widget::{canvas, Column, Scrollable};
use iced::window::icon::from_file;
use iced::{
    theme::Palette,
    widget::{
        button, container, pane_grid, svg::Svg, text, Button, Container, PaneGrid, Row, Stack,
    },
    Background, Border, Element, Length, Shadow, Theme,
};
use rfd::FileDialog;

mod circle;
mod color;
mod viewer;

use circle::Circle;
use color::Hex;
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

enum Pane {
    MainPane,
    ThemePane,
}

struct Icon(String, Message);

struct App {
    theme: Theme,
    icons: [Icon; 5],
    status: Status,
    panes: pane_grid::State<Pane>,
    focus: Option<pane_grid::Pane>,
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
        let (panes, focus) = pane_grid::State::new(Pane::MainPane);
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
            panes: panes,
            focus: Some(focus),
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
                        if let Some((_, siblings)) = self.panes.close(self.focus.unwrap()) {
                            self.focus = Some(siblings);
                        }
                    }
                    _ => {
                        self.status = Status::ChangeThemeLayout;
                        if let Some((siblings, split)) = self.panes.split(
                            pane_grid::Axis::Vertical,
                            self.focus.unwrap(),
                            Pane::ThemePane,
                        ) {
                            self.panes.resize(split, 0.9);
                            self.focus = Some(siblings);
                        }
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
        match self.status {
            Status::MainLayout => self.main_layout(),
            Status::ChangeThemeLayout => {
                PaneGrid::new(&self.panes, |_, state, _| {
                    pane_grid::Content::new(match state {
                        Pane::MainPane => self.main_layout(),
                        Pane::ThemePane => self.available_theme(),
                    })
                })
                .into()
                // Row::from_vec(vec![self.main_layout(), self.available_theme()])
                //     .width(Length::Shrink)
                //     .into()
            }
        }
    }

    fn main_layout(&self) -> Element<'_, Message> {
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
            .width(Length::Shrink)
            .padding(10.)
            .center_x(Length::Fill)
            .into(),
        ])
        .width(Length::Shrink)
        .into()
    }

    fn available_theme(&self) -> Element<'_, Message> {
        Column::from_vec(vec![
            text("Available themes").size(14).into(),
            Scrollable::new(
                Column::from_vec(
                    Theme::ALL
                        .iter()
                        .enumerate()
                        .map(|(i, theme)| {
                            Button::new(Row::from_vec(vec![
                                canvas(Circle {
                                    radius: 5.,
                                    color: theme.palette().primary,
                                })
                                .width(20.)
                                .height(20.)
                                .into(),
                                text(format!("{:?}", theme)).into(),
                            ]))
                            .width(200.)
                            .on_press(Message::SelectTheme(i))
                            .into()
                        })
                        .collect(),
                )
                .spacing(5.)
                .width(Length::Shrink)
                .height(Length::Shrink),
            )
            .width(Length::Shrink)
            .into(),
        ])
        .width(Length::Shrink)
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
