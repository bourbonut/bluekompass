use iced::theme::{Palette, Theme};
use iced::widget::{button, container};
use iced::{Background, Border, Shadow};

pub fn default_tool_style(palette: Palette) -> button::Style {
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

pub fn svg_tool_style(theme: &Theme, status: button::Status) -> button::Style {
    let base = default_tool_style(theme.palette());
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(theme.palette().primary.scale_alpha(0.5))),
            ..base
        },
        _ => base,
    }
}

pub fn theme_style(theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        border: Border::default().rounded(10.),
        text_color: theme.palette().text,
        ..Default::default()
    }
}

pub fn container_transparent(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(iced::Color::TRANSPARENT)),
        ..Default::default()
    }
}

pub fn container_theme_filled(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.palette().background)),
        ..Default::default()
    }
}
