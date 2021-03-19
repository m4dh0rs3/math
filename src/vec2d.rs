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
