use anyhow::{anyhow, Result};
use geo_types::{Geometry, Point};
use tui::style::Color;
use tui::widgets::canvas::{Painter, Points, Shape};

pub(crate) struct GeometryShape(Geometry<f64>);

impl From<Geometry<f64>> for GeometryShape {
    fn from(geometry: Geometry<f64>) -> Self {
        GeometryShape(geometry)
    }
}

impl Shape for GeometryShape {
    fn draw<'a, 'b>(&self, painter: &mut Painter<'a, 'b>) {
        match self.0 {
            Geometry::Point(point) => {
                let point_shape = Points {
                    coords: &[(point.0.x, point.0.y)],
                    color: Color::Red,
                };
                point_shape.draw(painter)
            }
            _ => {}
        }
    }
}

pub(crate) fn into_canvas_shape(geometry: Geometry<f64>) -> anyhow::Result<Box<dyn Shape>> {
    match geometry {
        Geometry::Point(_point) => Ok(Box::new(Points {
            coords: &[(10.0, 10.0)],
            color: Color::Red,
        })),
        _ => Err(anyhow!("unsupported geometry: {:?}", geometry)),
    }
}
