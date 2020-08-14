use crate::control::TU;
use nalgebra::Point2;
use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

/// A shape to draw a group of points with the given color
#[derive(Debug, Clone)]
pub struct GeoPoints {
    pub coords: Vec<(TU, TU)>,
    pub color: Color,
}

impl Shape for GeoPoints {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in &self.coords {
            if let Some((x, y)) = painter.get_point(*x, *y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
