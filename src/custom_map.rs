use crate::world::{WORLD_LOW_RESOLUTION, WORLD_HIGH_RESOLUTION};
use tui::{
    style::Color,
    widgets::canvas::{
        Painter, Shape
    },
};

#[derive(Debug, Clone, Copy)]
pub enum CustomMapResolution {
    Low,
    High,
}

impl CustomMapResolution {
    fn data(self) -> &'static [(f64, f64)] {
        match self {
            CustomMapResolution::Low => &WORLD_LOW_RESOLUTION,
            CustomMapResolution::High => &WORLD_HIGH_RESOLUTION,
        }
    }
}

/// Shape to draw a world map with the given resolution and color
#[derive(Debug, Clone)]
pub struct CustomMap {
    pub resolution: CustomMapResolution,
    pub color: Color,
}

impl Default for CustomMap {
    fn default() -> CustomMap {
        CustomMap {
            resolution: CustomMapResolution::Low,
            color: Color::Reset,
        }
    }
}

impl Shape for CustomMap {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.resolution.data() {
            if let Some((x, y)) = painter.get_point(*x, *y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
