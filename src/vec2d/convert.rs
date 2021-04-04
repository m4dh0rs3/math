use super::Vec2D;

macro_rules! from_float {
    ($Float: ty, $Integral: ty) => {
        impl From<Vec2D<$Float>> for Vec2D<$Integral> {
            fn from(vec_2d: Vec2D<$Float>) -> Self {
                Self {
                    x: vec_2d.x.round() as $Integral,
                    y: vec_2d.y.round() as $Integral,
                }
            }
        }
    };
}

macro_rules! from_integral {
    ($Integral: ty, $Float: ty) => {
        impl From<Vec2D<$Integral>> for Vec2D<$Float> {
            fn from(vec_2d: Vec2D<$Integral>) -> Self {
                Self {
                    x: vec_2d.x as $Float,
                    y: vec_2d.y as $Float,
                }
            }
        }
    };
}

from_integral!(f32, f64);
from_integral!(f64, f32);

from_float!(f64, i8);
from_float!(f64, i16);
from_float!(f64, i32);
from_float!(f64, i64);

from_float!(f32, i8);
from_float!(f32, i16);
from_float!(f32, i32);
from_float!(f32, i64);

from_integral!(i8, f64);
from_integral!(i16, f64);
from_integral!(i32, f64);
from_integral!(i64, f64);

from_integral!(i8, f32);
from_integral!(i16, f32);
from_integral!(i32, f32);
from_integral!(i64, f32);
