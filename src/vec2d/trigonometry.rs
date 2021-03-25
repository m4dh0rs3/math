use super::Vec2D;

macro_rules! trig {
    ($Float: ty) => {
        impl Vec2D<$Float> {
            pub fn dist(&self, rhs: Self) -> $Float {
                ((self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2)).sqrt()
            }

            pub fn maq(&self) -> $Float {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }

            pub fn norm(&mut self) {
                let maq = self.maq();

                self.x /= maq;
                self.y /= maq;
            }

            pub fn round(&self) -> Self {
                Self {
                    x: self.x.round(),
                    y: self.y.round(),
                }
            }

            /* pub fn angle(&self) -> $Float {
                self.y.atan2(self.x)
            } */

            pub fn from_polar(a: $Float, r: $Float) -> Self {
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
