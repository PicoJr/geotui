use crate::control::{ControlTransform, TU};
use geo_types::Geometry;
use nalgebra::Point2;
use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

pub struct GeoPoints {
    pub coords: Vec<(TU, TU)>,
    pub color: Color,
}

pub enum GeoShape {
    GeoPoints(GeoPoints),
}

impl Shape for GeoShape {
    fn draw<'a, 'b>(&self, painter: &mut Painter<'a, 'b>) {
        match self {
            GeoShape::GeoPoints(geo_points) => {
                for (x, y) in &geo_points.coords {
                    if let Some((x, y)) = painter.get_point(*x, *y) {
                        painter.paint(x, y, geo_points.color);
                    }
                }
            }
        }
    }
}

pub(crate) fn geometry_as_shape(
    geometry: &Geometry<TU>,
    transform: &ControlTransform,
) -> Option<GeoShape> {
    match geometry {
        Geometry::Point(point) => {
            let point = Point2::new(point.0.x, point.0.y);
            let transformed: Point2<f64> = transform * point;
            Some(GeoShape::GeoPoints(GeoPoints {
                coords: vec![(transformed.x, transformed.y)],
                color: Color::Red,
            }))
        }
        _ => None,
    }
}
