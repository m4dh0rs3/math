mod algebra;

/// Holds an value of radians in [-π; π].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle(f64);

impl From<f64> for Angle {
    fn from(float_64: f64) -> Self {
        Angle::new(float_64)
    }
}

impl Angle {
    /// Create a new angle from [`f64`].
    /// Value will be normalized! ([-π; π])
    pub fn new(angle: f64) -> Self {
        Self(angle).normal()
    }

    /// Normaize an [`Angle`] from [`f64`] to [-π; π].
    pub fn normal(self) -> Self {
        use std::f64::consts::PI;

        if *self < PI || *self > PI {
            Angle(self.sin().atan2(self.cos()))
        } else {
            self
        }
    }
}
