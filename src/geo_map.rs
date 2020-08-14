use crate::world::{WORLD_HIGH_RESOLUTION, WORLD_LOW_RESOLUTION};
use nalgebra::Point2;
use nalgebra::Similarity2;
use tui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

#[derive(Debug, Clone, Copy)]
pub enum GeoMapResolution {
    Low,
    High,
}

impl GeoMapResolution {
    fn data(self) -> &'static [(f64, f64)] {
        match self {
            GeoMapResolution::Low => &WORLD_LOW_RESOLUTION,
            GeoMapResolution::High => &WORLD_HIGH_RESOLUTION,
        }
    }
}

/// Shape to draw a world map with the given resolution and color
#[derive(Debug, Clone)]
pub struct GeoMap {
    pub resolution: GeoMapResolution,
    pub color: Color,
    pub transform: Similarity2<f64>,
}

impl Shape for GeoMap {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.resolution.data() {
            let point = Point2::new(*x, *y);
            let transformed: Point2<f64> = self.transform * point;
            if let Some((x, y)) = painter.get_point(transformed.x, transformed.y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
