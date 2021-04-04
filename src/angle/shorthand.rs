use super::Angle;
use std::f64::consts::{FRAC_2_PI, PI};

impl Angle {
    /// Create new [`Angle`] pointing up.
    fn up() -> Self {
        Self::raw(-PI)
    }

    /// Create new [`Angle`] pointing left.
    fn left() -> Self {
        Self::raw(FRAC_2_PI)
    }

    /// Create new [`Angle`] pointing right.
    fn right() -> Self {
        Self::raw(-FRAC_2_PI)
    }

    /// Create new [`Angle`] pointing down.
    fn down() -> Self {
        Self::raw(0.0)
    }
}
