use nalgebra::{Similarity2, Vector2};
use termion::event::Key;

pub(crate) type TU = f64;
pub(crate) type ControlTransform = Similarity2<TU>;

pub(crate) fn control(key: &Key) -> Similarity2<f64> {
    match key {
        Key::PageUp => Similarity2::from_scaling(1.1),
        Key::PageDown => Similarity2::from_scaling(0.9),
        Key::Left => Similarity2::new(Vector2::new(2.0, 0.0), 0.0, 1.0),
        Key::Right => Similarity2::new(Vector2::new(-2.0, 0.0), 0.0, 1.0),
        Key::Down => Similarity2::new(Vector2::new(0.0, 2.0), 0.0, 1.0),
        Key::Up => Similarity2::new(Vector2::new(0.0, -2.0), 0.0, 1.0),
        _ => Similarity2::identity(),
    }
}
