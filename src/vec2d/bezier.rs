use std::ops;

pub fn remap<
    T: ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + Copy,
>(
    x: T,
    a: T,
    b: T,
    c: T,
    d: T,
) -> T {
    x / (b - a) * (d - c) + c
}

pub fn lerp<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy>(
    t: T,
    a: T,
    b: T,
) -> T {
    a + t * (b - a)
}

pub fn bezier<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy>(
    t: T,
    a: T,
    b: T,
    c: T,
) -> T {
    lerp(t, lerp(t, a, b), lerp(t, b, c))
}

use super::Vec2D;

impl<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy> Vec2D<T> {
    pub fn lerp(t: T, a: Self, b: Self) -> Self {
        Self {
            x: lerp(t, a.x, b.x),
            y: lerp(t, a.y, b.y),
        }
    }

    pub fn bezier(t: T, a: Self, b: Self, c: Self) -> Self {
        Self {
            x: bezier(t, a.x, b.x, c.x),
            y: bezier(t, a.y, b.y, c.y),
        }
    }
}
