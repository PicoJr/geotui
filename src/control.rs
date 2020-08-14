use nalgebra::{Similarity2, Vector2};
use termion::event::Key;

static DELTA_LON_LAT: f64 = 2.0;
static SCALING_UP: f64 = 1.2;
static SCALING_DOWN: f64 = 0.80;

pub(crate) type TU = f64;
pub(crate) type ControlTransform = Similarity2<TU>;

pub(crate) fn control(key: &Key) -> Similarity2<f64> {
    match key {
        Key::PageUp => Similarity2::from_scaling(SCALING_UP),
        Key::PageDown => Similarity2::from_scaling(SCALING_DOWN),
        Key::Left => Similarity2::new(Vector2::new(DELTA_LON_LAT, 0.0), 0.0, 1.0),
        Key::Right => Similarity2::new(Vector2::new(-DELTA_LON_LAT, 0.0), 0.0, 1.0),
        Key::Down => Similarity2::new(Vector2::new(0.0, DELTA_LON_LAT), 0.0, 1.0),
        Key::Up => Similarity2::new(Vector2::new(0.0, -DELTA_LON_LAT), 0.0, 1.0),
        _ => Similarity2::identity(),
    }
}
