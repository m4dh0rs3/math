use crate::{func::*, vec2d::Vec2D};

#[test]
fn vec2d_ops() {
    assert_eq!(3.0 * Vec2D::new(2.0, 3.0), Vec2D::new(6.0, 9.0));
    assert_eq!(Vec2D::new(2.0, 3.0) * 3.0, Vec2D::new(6.0, 9.0));
    assert_eq!(Vec2D::new(2.0, 3.0) * 0.5, Vec2D::new(1.0, 1.5));
    assert_eq!(Vec2D::new(2.0, 3.0) / 3.0, Vec2D::new(2.0 / 3.0, 1.0));
    assert_eq!(
        Vec2D::new(2.0, 3.0) + Vec2D::new(3.0, 4.0),
        Vec2D::new(5.0, 7.0)
    );
    assert_eq!(
        Vec2D::new(2.0, 3.0) - Vec2D::new(3.0, 1.0),
        Vec2D::new(-1.0, 2.0)
    );

    let mut p = Vec2D::new(2.0, 3.0);
    p += Vec2D::new(1.0, 2.0);
    assert_eq!(p, Vec2D::new(3.0, 5.0));
    p *= 0.5;
    assert_eq!(p, Vec2D::new(1.5, 2.5));
    p /= 0.5;
    assert_eq!(p, Vec2D::new(3.0, 5.0));

    assert_eq!(Vec2D::from_polar(0.0, 1.0), Vec2D::new(1.0, 0.0));
    assert_eq!(Vec2D::from_polar(0.0, 2.0), Vec2D::new(2.0, 0.0));
    assert_eq!(
        Vec2D::from_polar(std::f64::consts::PI, 1.0).angle(),
        std::f64::consts::PI
    );

    assert_eq!(
        {
            let mut p = Vec2D::new(2.0, 0.0);
            p.norm();
            p
        },
        Vec2D::new(1.0, 0.0)
    );

    assert_eq!(Vec2D::new(3.0, 4.0).maq(), 5.0);

    assert_eq!(Vec2D::new(3.0, 4.0).dist(&Vec2D::new(0.0, 0.0)), 5.0);
    assert_eq!(
        Vec2D::new(3.0, 4.0).dist(&Vec2D::new(1.0, 1.0)),
        13.0_f64.sqrt()
    );
}

#[test]
fn vec2d_utils() {
    assert_eq!(
        Vec2D::lerp(0.5, Vec2D::new(0.0, 0.0), Vec2D::new(2.0, 1.0)),
        Vec2D::new(1.0, 0.5)
    );

    assert_eq!(
        Vec2D::lerp(1.0, Vec2D::new(0.0, 0.0), Vec2D::new(2.0, 1.0)),
        Vec2D::new(2.0, 1.0)
    );

    assert_eq!(
        Vec2D::bezier(
            0.5,
            Vec2D::new(0.0, 0.0),
            Vec2D::new(2.0, 4.0),
            Vec2D::new(4.0, 3.0),
        ),
        Vec2D::new(2.5, 2.5)
    );
}

#[test]
fn utils_fn() {
    assert_eq!(remap(5.0, 0.0, 10.0, 0.0, 5.0), 2.5);
    assert_eq!(remap(-5.0, 0.0, 10.0, 0.0, 5.0), -2.5);
    assert_eq!(remap(0.5, 0.0, 1.0, 0.0, 360.0), 180.0);

    assert_eq!(lerp(0.25, -1.0, 3.0), 0.0);
    assert_eq!(lerp(0.5, 0.0, 3.0), 1.5);

    assert_eq!(bezier(0.0, 2.0, 4.0, 6.0), 2.0);
    assert_eq!(bezier(0.5, 0.0, 4.0, 6.0), 4.0);
    assert_eq!(bezier(1.0, 2.0, 4.0, 6.0), 6.0);
}

#[test]
fn intersection() {
    assert_eq!(
        Vec2D::intersect(
            Vec2D::new(-2.0, -2.0),
            Vec2D::new(2.0, 2.0),
            Vec2D::new(-2.0, 2.0),
            Vec2D::new(2.0, -2.0),
        ),
        Some(Vec2D::new(0.0, 0.0)),
    );
    assert_eq!(
        Vec2D::intersect(
            Vec2D::new(0.0, 0.0),
            Vec2D::new(1.0, 1.0),
            Vec2D::new(1.0, 1.0),
            Vec2D::new(2.0, 0.0),
        ),
        Some(Vec2D::new(1.0, 1.0)),
    );
    assert_eq!(
        Vec2D::intersect(
            Vec2D::new(0.0, 0.0),
            Vec2D::new(1.0, 2.0),
            Vec2D::new(1.0, 1.0),
            Vec2D::new(2.0, 0.0),
        ),
        None,
    );
}
