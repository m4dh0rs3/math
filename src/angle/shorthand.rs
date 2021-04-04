use super::Angle;
use std::f64::consts::{FRAC_2_PI, PI};

impl Angle {
    /// Create new [`Angle`] pointing up.
    pub fn up() -> Self {
        Self::raw(-PI)
    }

    /// Create new [`Angle`] pointing left.
    pub fn left() -> Self {
        Self::raw(FRAC_2_PI)
    }

    /// Create new [`Angle`] pointing right.
    pub fn right() -> Self {
        Self::raw(-FRAC_2_PI)
    }

    /// Create new [`Angle`] pointing down.
    pub fn down() -> Self {
        Self::raw(0.0)
    }
}
