use crate::vec2d::{bezier::*, *};

#[test]
fn vec2df64_ops() {
    assert_eq!(
        Vec2D::<f64>::new(2.0, 3.0) * 3.0,
        Vec2D::<f64>::new(6.0, 9.0)
    );
    assert_eq!(
        Vec2D::<f64>::new(2.0, 3.0) * 3.0,
        Vec2D::<f64>::new(6.0, 9.0)
    );
    assert_eq!(
        Vec2D::<f64>::new(2.0, 3.0) * 0.5,
        Vec2D::<f64>::new(1.0, 1.5)
    );
    assert_eq!(
        Vec2D::<f64>::new(2.0, 3.0) / 3.0,
        Vec2D::<f64>::new(2.0 / 3.0, 1.0)
    );
    assert_eq!(
        Vec2D::<f64>::new(2.0, 3.0) + Vec2D::<f64>::new(3.0, 4.0),
        Vec2D::<f64>::new(5.0, 7.0)
    );
    assert_eq!(
        Vec2D::<f64>::new(2.0, 3.0) - Vec2D::<f64>::new(3.0, 1.0),
        Vec2D::<f64>::new(-1.0, 2.0)
    );

    let mut p = Vec2D::<f64>::new(2.0, 3.0);
    p += Vec2D::<f64>::new(1.0, 2.0);
    assert_eq!(p, Vec2D::<f64>::new(3.0, 5.0));
    p *= 0.5;
    assert_eq!(p, Vec2D::<f64>::new(1.5, 2.5));
    p /= 0.5;
    assert_eq!(p, Vec2D::<f64>::new(3.0, 5.0));

    assert_eq!(
        Vec2D::<f64>::from_polar(0.0, 1.0),
        Vec2D::<f64>::new(1.0, 0.0)
    );
    assert_eq!(
        Vec2D::<f64>::from_polar(0.0, 2.0),
        Vec2D::<f64>::new(2.0, 0.0)
    );
    /* assert_eq!(
        Vec2D::<f64>::from_polar(std::f64::consts::PI, 1.0).angle(),
        std::f64::consts::PI
    ); */

    assert_eq!(
        {
            let mut p = Vec2D::<f64>::new(2.0, 0.0);
            p.norm();
            p
        },
        Vec2D::<f64>::new(1.0, 0.0)
    );

    assert_eq!(Vec2D::<f64>::new(3.0, 4.0).maq(), 5.0);

    assert_eq!(
        Vec2D::<f64>::new(3.0, 4.0).dist(Vec2D::<f64>::new(0.0, 0.0)),
        5.0
    );
    assert_eq!(
        Vec2D::<f64>::new(3.0, 4.0).dist(Vec2D::<f64>::new(1.0, 1.0)),
        13.0_f64.sqrt()
    );
}

#[test]
fn vec2df64_utils() {
    assert_eq!(
        Vec2D::<f64>::lerp(
            0.5,
            Vec2D::<f64>::new(0.0, 0.0),
            Vec2D::<f64>::new(2.0, 1.0)
        ),
        Vec2D::<f64>::new(1.0, 0.5)
    );

    assert_eq!(
        Vec2D::<f64>::lerp(
            1.0,
            Vec2D::<f64>::new(0.0, 0.0),
            Vec2D::<f64>::new(2.0, 1.0)
        ),
        Vec2D::<f64>::new(2.0, 1.0)
    );

    assert_eq!(
        Vec2D::<f64>::bezier(
            0.5,
            Vec2D::<f64>::new(0.0, 0.0),
            Vec2D::<f64>::new(2.0, 4.0),
            Vec2D::<f64>::new(4.0, 3.0),
        ),
        Vec2D::<f64>::new(2.0, 2.75)
    );
}

#[test]
fn utils_fn_f64() {
    assert_eq!(remap(5.0, 0.0, 10.0, 0.0, 5.0), 2.5);
    assert_eq!(remap(-5.0, 0.0, 10.0, 0.0, 5.0), -2.5);
    assert_eq!(remap(0.5, 0.0, 1.0, 0.0, 360.0), 180.0);

    assert_eq!(lerp(0.25, -1.0, 3.0), 0.0);
    assert_eq!(lerp(0.5, 0.0, 3.0), 1.5);

    assert_eq!(bezier(0.0, 2.0, 4.0, 6.0), 2.0);
    assert_eq!(bezier(0.5, 0.0, 4.0, 6.0), 3.5);
    assert_eq!(bezier(1.0, 2.0, 4.0, 6.0), 6.0);
}

#[test]
fn intersection_f64() {
    assert_eq!(
        Vec2D::<f64>::intersect(
            Vec2D::<f64>::new(-2.0, -2.0),
            Vec2D::<f64>::new(2.0, 2.0),
            Vec2D::<f64>::new(-2.0, 2.0),
            Vec2D::<f64>::new(2.0, -2.0),
        ),
        Some(Vec2D::<f64>::new(0.0, 0.0)),
    );
    assert_eq!(
        Vec2D::<f64>::intersect(
            Vec2D::<f64>::new(0.0, 0.0),
            Vec2D::<f64>::new(1.0, 1.0),
            Vec2D::<f64>::new(1.0, 1.0),
            Vec2D::<f64>::new(2.0, 0.0),
        ),
        Some(Vec2D::<f64>::new(1.0, 1.0)),
    );
    assert_eq!(
        Vec2D::<f64>::intersect(
            Vec2D::<f64>::new(0.0, 0.0),
            Vec2D::<f64>::new(1.0, 2.0),
            Vec2D::<f64>::new(1.0, 1.0),
            Vec2D::<f64>::new(2.0, 0.0),
        ),
        None,
    );
}
/*
#[test]
fn vec2df32_ops() {
    assert_eq!(
        Vec2D::<f32>::new(2.0, 3.0) * 3.0,
        Vec2D::<f32>::new(6.0, 9.0)
    );
    assert_eq!(
        Vec2D::<f32>::new(2.0, 3.0) * 3.0,
        Vec2D::<f32>::new(6.0, 9.0)
    );
    assert_eq!(
        Vec2D::<f32>::new(2.0, 3.0) * 0.5,
        Vec2D::<f32>::new(1.0, 1.5)
    );
    assert_eq!(
        Vec2D::<f32>::new(2.0, 3.0) / 3.0,
        Vec2D::<f32>::new(2.0 / 3.0, 1.0)
    );
    assert_eq!(
        Vec2D::<f32>::new(2.0, 3.0) + Vec2D::<f32>::new(3.0, 4.0),
        Vec2D::<f32>::new(5.0, 7.0)
    );
    assert_eq!(
        Vec2D::<f32>::new(2.0, 3.0) - Vec2D::<f32>::new(3.0, 1.0),
        Vec2D::<f32>::new(-1.0, 2.0)
    );

    let mut p = Vec2D::<f32>::new(2.0, 3.0);
    p += Vec2D::<f32>::new(1.0, 2.0);
    assert_eq!(p, Vec2D::<f32>::new(3.0, 5.0));
    p *= 0.5;
    assert_eq!(p, Vec2D::<f32>::new(1.5, 2.5));
    p /= 0.5;
    assert_eq!(p, Vec2D::<f32>::new(3.0, 5.0));

    assert_eq!(
        Vec2D::<f32>::from_polar(0.0, 1.0),
        Vec2D::<f32>::new(1.0, 0.0)
    );
    assert_eq!(
        Vec2D::<f32>::from_polar(0.0, 2.0),
        Vec2D::<f32>::new(2.0, 0.0)
    );
    /* assert_eq!(
        Vec2D::<f32>::from_polar(std::f32::consts::PI, 1.0).angle(),
        std::f32::consts::PI
    ); */

    assert_eq!(
        {
            let mut p = Vec2D::<f32>::new(2.0, 0.0);
            p.norm();
            p
        },
        Vec2D::<f32>::new(1.0, 0.0)
    );

    assert_eq!(Vec2D::<f32>::new(3.0, 4.0).maq(), 5.0);

    assert_eq!(
        Vec2D::<f32>::new(3.0, 4.0).dist(Vec2D::<f32>::new(0.0, 0.0)),
        5.0
    );
    assert_eq!(
        Vec2D::<f32>::new(3.0, 4.0).dist(Vec2D::<f32>::new(1.0, 1.0)),
        13.0_f32.sqrt()
    );
}

#[test]
fn vec2df32_utils() {
    assert_eq!(
        Vec2D::<f32>::lerp(
            0.5,
            Vec2D::<f32>::new(0.0, 0.0),
            Vec2D::<f32>::new(2.0, 1.0)
        ),
        Vec2D::<f32>::new(1.0, 0.5)
    );

    assert_eq!(
        Vec2D::<f32>::lerp(
            1.0,
            Vec2D::<f32>::new(0.0, 0.0),
            Vec2D::<f32>::new(2.0, 1.0)
        ),
        Vec2D::<f32>::new(2.0, 1.0)
    );

    assert_eq!(
        Vec2D::<f32>::bezier(
            0.5,
            Vec2D::<f32>::new(0.0, 0.0),
            Vec2D::<f32>::new(2.0, 4.0),
            Vec2D::<f32>::new(4.0, 3.0),
        ),
        Vec2D::<f32>::new(2.0, 2.75)
    );
}

#[test]
fn utils_fn_f32() {
    assert_eq!(remap(5.0, 0.0, 10.0, 0.0, 5.0), 2.5);
    assert_eq!(remap(-5.0, 0.0, 10.0, 0.0, 5.0), -2.5);
    assert_eq!(remap(0.5, 0.0, 1.0, 0.0, 360.0), 180.0);

    assert_eq!(lerp(0.25, -1.0, 3.0), 0.0);
    assert_eq!(lerp(0.5, 0.0, 3.0), 1.5);

    assert_eq!(bezier(0.0, 2.0, 4.0, 6.0), 2.0);
    assert_eq!(bezier(0.5, 0.0, 4.0, 6.0), 3.5);
    assert_eq!(bezier(1.0, 2.0, 4.0, 6.0), 6.0);
}

#[test]
fn intersection_f32() {
    assert_eq!(
        Vec2D::<f32>::intersect(
            Vec2D::<f32>::new(-2.0, -2.0),
            Vec2D::<f32>::new(2.0, 2.0),
            Vec2D::<f32>::new(-2.0, 2.0),
            Vec2D::<f32>::new(2.0, -2.0),
        ),
        Some(Vec2D::<f32>::new(0.0, 0.0)),
    );
    assert_eq!(
        Vec2D::<f32>::intersect(
            Vec2D::<f32>::new(0.0, 0.0),
            Vec2D::<f32>::new(1.0, 1.0),
            Vec2D::<f32>::new(1.0, 1.0),
            Vec2D::<f32>::new(2.0, 0.0),
        ),
        Some(Vec2D::<f32>::new(1.0, 1.0)),
    );
    assert_eq!(
        Vec2D::<f32>::intersect(
            Vec2D::<f32>::new(0.0, 0.0),
            Vec2D::<f32>::new(1.0, 2.0),
            Vec2D::<f32>::new(1.0, 1.0),
            Vec2D::<f32>::new(2.0, 0.0),
        ),
        None,
    );
}
 */
