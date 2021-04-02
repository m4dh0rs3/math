/// Holds an value of radians in [-π; π].
#[derive(Debug, Clone)]
pub struct Angle(f64);

// to call [`f64`] methods on it
impl std::ops::Deref for Angle {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
        Angle(self.sin().atan2(self.cos()))
    }
}
