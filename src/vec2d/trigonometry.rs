use super::Vec2D;
use crate::angle::Angle;

macro_rules! trig {
    ($Float: ty) => {
        impl Vec2D<$Float> {
            pub fn dist(&self, rhs: Self) -> $Float {
                ((self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2)).sqrt()
            }

            pub fn maq(&self) -> $Float {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }

            pub fn normal(&mut self) -> Self {
                let maq = self.maq();

                Self {
                    x: self.x / maq,
                    y: self.y / maq,
                }
            }

            pub fn round(&self) -> Self {
                Self {
                    x: self.x.round(),
                    y: self.y.round(),
                }
            }

            pub fn angle(&self) -> Angle {
                self.y.atan2(self.x).into()
            }

            pub fn from_polar(a: Angle, r: $Float) -> Self {
                Self {
                    x: r * a.cos(),
                    y: r * a.sin(),
                }
            }
        }
    };
}

//trig!(f32);
trig!(f64);
