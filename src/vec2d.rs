pub(crate) mod algebra;
pub(crate) mod bezier;
pub(crate) mod intersect;
pub(crate) mod trigonometry;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl From<Vec2D<i8>> for Vec2D<f64> {
    fn from(vec_2d: Vec2D<i8>) -> Self {
        Self {
            x: vec_2d.x.into(),
            y: vec_2d.y.into(),
        }
    }
}

impl From<Vec2D<f64>> for Vec2D<i8> {
    fn from(vec_2d: Vec2D<f64>) -> Self {
        Self {
            x: vec_2d.x.round() as i8,
            y: vec_2d.y.round() as i8,
        }
    }
}
