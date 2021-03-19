use super::Vec2D;
use std::ops;

impl<T: ops::Mul<Output = T> + ops::Sub<Output = T> + Copy> Vec2D<T> {
    pub fn cross_zero(&self, rhs: Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl<
        T: ops::Mul<Output = T>
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + PartialEq
            + PartialOrd
            + From<u8>
            + Copy,
    > Vec2D<T>
{
    pub fn intersect(p1: Self, p2: Self, r1: Self, r2: Self) -> Option<Self> {
        let s1 = p2 - p1;
        let s2 = r2 - r1;

        let k = s1.cross_zero(s2);

        if k == 0.into() {
            return None;
        }

        let d = p1 - r1;

        let s = s1.cross_zero(d) / k;
        let t = s2.cross_zero(d) / k;

        /* let s = (-s1.y * (p1.x - r1.x) + s1.x * (p1.y - r1.y)) / k;
        let t = (s2.x * (p1.y - r1.y) - s2.y * (p1.x - r1.x)) / k; */

        if s >= 0.into() && s <= 1.into() && t >= 0.into() && t <= 1.into() {
            Some(Self {
                x: p1.x + t * s1.x,
                y: p1.y + t * s1.y,
            })
        } else {
            None
        }
    }
}
