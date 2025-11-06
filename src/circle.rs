use iced::mouse;
use iced::widget::canvas;
use iced::Color;
use iced::Rectangle;
use iced::Renderer;
use iced::Theme;

#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
    pub border_radius: f32,
    pub fill_color: Color,
    pub border_color: Color,
}

// Then, we implement the `Program` trait
impl<Message> canvas::Program<Message> for Circle {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // We create a `Path` representing a simple circle
        let filled_circle = canvas::Path::circle(frame.center(), self.radius);
        let border_circle = canvas::Path::circle(frame.center(), self.radius + self.border_radius);

        // And fill it with some color
        frame.fill(&border_circle, self.border_color);
        frame.fill(&filled_circle, self.fill_color);

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}
