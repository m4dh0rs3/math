mod algebra;
mod bezier;
mod convert;
mod intersect;
mod trigonometry;

/// 2-Dimensional vector containing [`T`] on x and y.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2D<T> {
    /// Create a new [`Vec2D`].
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: From<u8>> Vec2D<T> {
    /// Create new [`Vec2D`] at origin.
    pub fn zero() -> Self {
        Self::new(0.into(), 0.into())
    }
}
