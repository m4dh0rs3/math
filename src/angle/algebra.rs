use super::Angle;
use std::ops;

// to call [`f64`] methods on it
impl std::ops::Deref for Angle {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Angle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ops::Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(*self + *rhs)
    }
}

impl ops::Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(*self - *rhs)
    }
}

impl ops::Mul for Angle {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(*self * *rhs)
    }
}

impl ops::Div for Angle {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(*self / *rhs)
    }
}
