use egui_plot::{PlotPoint, PlotUi};
use crate::shapes::Shape;

mod line;
mod circle;

pub use line::Line;
pub use circle::Circle;

pub trait ShapeBuilder {
    fn build(&self, points: &Vec<PlotPoint>) -> Option<Box<dyn Shape>>;
    fn draw(&self, points: &Vec<PlotPoint>, plot_ui: &mut PlotUi, current_point: PlotPoint);
}

pub struct Builder {
    points: Vec<PlotPoint>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.points.clear();
    }

    pub fn set_next_point(&mut self, point: PlotPoint) {
        if self.points.len() < 3 {
            self.points.push(point);
        }
    }

    pub fn draw<T: ShapeBuilder>(&self, plot_ui: &mut PlotUi, current_point: PlotPoint, shape: T) { 
        shape.draw(&self.points, plot_ui, current_point);
    }

    pub fn build<T: ShapeBuilder>(&self, shape: T) -> Option<Box<dyn Shape>> {
        shape.build(&self.points)
    }
}
