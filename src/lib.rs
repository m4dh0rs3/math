pub mod angle;
pub mod bezier;
pub mod vec2d;

pub mod prelude {
    pub use crate::{angle::*, bezier::*, vec2d::Vec2D};
}

#[cfg(test)]
mod tests;
