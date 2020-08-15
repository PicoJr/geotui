use crate::control::{ControlTransform, TU};
use crate::geo_points::GeoPoints;
use geo_types::{Geometry, Polygon};
use nalgebra::Point2;
use tui::style::Color;
use tui::widgets::canvas::{Line, Painter, Shape};

pub enum GeoShape {
    GeoPoints(GeoPoints),
    GeoLine(Line),
    GeoPolygon(Vec<Line>),
}

impl Shape for GeoShape {
    fn draw<'a, 'b>(&self, painter: &mut Painter<'a, 'b>) {
        match self {
            GeoShape::GeoPoints(geo_points) => {
                geo_points.draw(painter);
            }
            GeoShape::GeoLine(geo_line) => {
                geo_line.draw(painter);
            }
            GeoShape::GeoPolygon(geo_lines) => {
                for geo_line in geo_lines {
                    geo_line.draw(painter);
                }
            }
        }
    }
}

fn transform_line(line: Line, transform: &ControlTransform) -> Line {
    let point_start = Point2::new(line.x1, line.y1);
    let transformed_start: Point2<f64> = transform * point_start;
    let point_end = Point2::new(line.x2, line.y2);
    let transformed_end: Point2<f64> = transform * point_end;
    Line {
        x1: transformed_start.x,
        y1: transformed_start.y,
        x2: transformed_end.x,
        y2: transformed_end.y,
        color: line.color,
    }
}

fn transform_polygon(polygon: &Polygon<TU>, transform: &ControlTransform) -> Vec<Line> {
    polygon
        .exterior()
        .lines()
        .map(|line| {
            transform_line(
                Line {
                    x1: line.start.x,
                    y1: line.start.y,
                    x2: line.end.x,
                    y2: line.end.y,
                    color: Color::Red,
                },
                transform,
            )
        })
        .collect()
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
        Geometry::Line(line) => Some(GeoShape::GeoLine(transform_line(
            Line {
                x1: line.start.x,
                y1: line.start.y,
                x2: line.end.x,
                y2: line.end.y,
                color: Color::Red,
            },
            transform,
        ))),
        Geometry::Polygon(polygon) => {
            Some(GeoShape::GeoPolygon(transform_polygon(polygon, transform)))
        }
        Geometry::MultiPolygon(polygons) => Some(GeoShape::GeoPolygon(
            polygons
                .0
                .iter()
                .flat_map(|polygon| transform_polygon(polygon, transform))
                .collect(),
        )),
        _ => None,
    }
}
