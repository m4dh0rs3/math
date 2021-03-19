use super::Vec2D;
use std::ops;

impl<T: ops::Add<Output = T>> ops::Add for Vec2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: ops::AddAssign> ops::AddAssign for Vec2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub for Vec2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: ops::SubAssign> ops::SubAssign for Vec2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: ops::Mul<Output = T>> ops::Mul for Vec2D<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: ops::MulAssign> ops::MulAssign for Vec2D<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<T> for Vec2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: ops::MulAssign + Copy> ops::MulAssign<T> for Vec2D<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/* impl ops::Mul<Vec2D<f64>> for f64 {
    type Output = Vec2D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
} */

impl<T: ops::Div<Output = T>> ops::Div for Vec2D<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div<T> for Vec2D<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: ops::DivAssign + Copy> ops::DivAssign<T> for Vec2D<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

/* impl ops::Div<Vec2D<f64>> for f64 {
    type Output = Vec2D<f64>;

    fn div(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
} */
