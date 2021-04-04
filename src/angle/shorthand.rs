use super::Angle;
use std::f64::consts::{FRAC_PI_2, PI};

impl Angle {
    /// Create new [`Angle`] pointing up.
    pub fn up() -> Self {
        Self::raw(-PI)
    }

    /// Create new [`Angle`] pointing left.
    pub fn left() -> Self {
        Self::raw(FRAC_PI_2)
    }

    /// Create new [`Angle`] pointing right.
    pub fn right() -> Self {
        Self::raw(-FRAC_PI_2)
    }

    /// Create new [`Angle`] pointing down.
    pub fn down() -> Self {
        Self::raw(0.0)
    }
}
