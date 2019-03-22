extern crate ranger;

use ranger::geometry::point::Point;
use ranger::geometry::rectangle::Rectangle;
use ranger::math::affine_transform::AffineTransform;
use ranger::math::vector2::Vector2;

const EPISION: f64 = 0.000001;
const FORTY_FIVE: f64 = 0.70710677;

#[test]
fn math_create_vector() {
    let v = Vector2::new(1.0, 0.0);

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 0.0);
}

#[test]
fn math_rotate_vector_45_ref() {
    let v = Vector2::new(1.0, 0.0);
    let mut at = AffineTransform::new();

    // By default +y-axis is downward to this rotation is CW.
    at.make_rotate(f64::to_radians(45.0));
    let vr = AffineTransform::transform_vector(&v, &at);

    assert_eq!(vr.x, 0.70710677);
    assert_eq!(vr.y, 0.70710677);

    // println!("vr: {},{}", vr.x, vr.y);
}

#[test]
fn math_rotate_vector_90_ref() {
    let v = Vector2::new(1.0, 0.0);
    let mut at = AffineTransform::new();

    // After rotation vector should be pointing downward.
    // .--------> X  (1,0)
    // |
    // |
    // |
    // v +Y (0,1)  ==> vr
    at.make_rotate(f64::to_radians(90.0));
    let vr = AffineTransform::transform_vector(&v, &at);
    // println!("vr: {},{}", (vr.x).round(), vr.y);

    assert_eq!(vr.x.round(), 0.0);
    assert_eq!(vr.y, 1.0);
}

#[test]
fn math_rotate_vector_neg90_ref() {
    let v = Vector2::new(1.0, 0.0);
    let mut at = AffineTransform::new();

    // After rotation vector should be pointing upward.
    // ^  -Y (0,-1)
    // |
    // |
    // |
    // .--------> X  (1,0)
    at.make_rotate(f64::to_radians(-90.0));
    let vr = AffineTransform::transform_vector(&v, &at);
    // println!("vr: {},{}", (vr.x).round(), vr.y.round());

    assert_eq!(vr.x.round(), 0.0);
    assert_eq!(vr.y, -1.0);
}

#[test]
fn math_rotate_vector_45twice_ref() {
    let v = Vector2::new(1.0, 0.0);
    let mut at = AffineTransform::new();

    // After rotation vector should be pointing downward.
    // .--------> X  (1,0)
    // |
    // |
    // |
    // v +Y (0,1)  ==> vr
    at.make_rotate(f64::to_radians(45.0));
    let vr = AffineTransform::transform_vector(&v, &at);
    let vr = AffineTransform::transform_vector(&vr, &at);
    // println!("vr: {},{}", (vr.x).round(), vr.y.round());

    assert_eq!(vr.x.round(), 0.0);
    assert_eq!(vr.y.round(), 1.0);
}

#[test]
fn math_rotate_vector_cat90_ref() {
    let v = Vector2::new(1.0, 0.0);
    let mut at = AffineTransform::new();

    // Concatenate two 45 degree rotations.
    // After rotation vector should be pointing downward.
    // .--------> X  (1,0)
    // |
    // |
    // |
    // v +Y (0,1)  ==> vr
    at.make_rotate(f64::to_radians(45.0));
    // Cat on another 45 degrees.
    at.rotate(f64::to_radians(45.0));
    let vr = AffineTransform::transform_vector(&v, &at);

    // println!("vr: {},{}", (vr.x).round(), vr.y.round());

    assert_eq!(vr.x.round(), 0.0);
    assert_eq!(vr.y.round(), 1.0);
}

#[test]
fn math_rotate_vector_380_at_45_steps_ref() {
    // Begin with the test vector along the +X axis.
    let v = Vector2::new(1.0, 0.0);
    let mut at = AffineTransform::new();

    // Rotate X vector at 45 degree increments in a clockwise rotation.
    // The reference coordinate system
    //             ^  -Y (0,-1)
    //             |
    //             |
    //             |
    // -X <--------.--------> +X  (1,0)
    //             |
    //             |
    //             |
    //             v +Y (0,1)

    at.make_rotate(f64::to_radians(45.0));
    let vr = AffineTransform::transform_vector(&v, &at);
    assert!(equal_abs(vr.x, FORTY_FIVE));
    assert!(equal_abs(vr.y, FORTY_FIVE));

    at.rotate(f64::to_radians(45.0));
    let vr = AffineTransform::transform_vector(&v, &at);
    assert_eq!(vr.x.round(), 0.0);
    assert_eq!(vr.y.round(), 1.0);

    at.rotate(f64::to_radians(45.0));
    let vr = AffineTransform::transform_vector(&v, &at);
    assert!(vr.x < 0.0);
    assert!(equal_abs(vr.x, FORTY_FIVE));
    assert!(equal_abs(vr.y, FORTY_FIVE));

    at.rotate(f64::to_radians(45.0));
    let vr = AffineTransform::transform_vector(&v, &at);
    // println!("vr: {},{}", vr.x, vr.y);
    assert!(vr.x < 0.0);
    assert!(equal_abs(vr.x, 1.0));
    assert!(equal_abs(vr.y, 0.0));

    at.rotate(f64::to_radians(45.0));
    let vr = AffineTransform::transform_vector(&v, &at);
    // println!("vr: {},{}", vr.x, vr.y);
    assert!(vr.x < 0.0);
    assert!(equal_abs(vr.x, FORTY_FIVE));
    assert!(vr.y < 0.0);
    assert!(equal_abs(vr.y, FORTY_FIVE));

    at.rotate(f64::to_radians(45.0));
    let vr = AffineTransform::transform_vector(&v, &at);
    // println!("vr: {},{}", vr.x, vr.y);
    assert!(equal_abs(vr.x, 0.0));
    assert!(vr.y < 0.0);
    assert!(equal_abs(vr.y, 1.0));

    at.rotate(f64::to_radians(45.0));
    let vr = AffineTransform::transform_vector(&v, &at);
    assert!(equal_abs(vr.x, FORTY_FIVE));
    assert!(vr.y < 0.0);
    assert!(equal_abs(vr.y, FORTY_FIVE));

    // Finally back to where we started, a vector along the X axis.
    at.rotate(f64::to_radians(45.0));
    let vr = AffineTransform::transform_vector(&v, &at);
    assert!(equal_abs(vr.x, 1.0));
    assert!(equal_abs(vr.y, 0.0));
}

fn equal_abs(v: f64, e: f64) -> bool {
    let dif = v.abs() - e.abs();
    // println!("dif: {}", dif.abs());
    dif.abs() < EPISION
}

#[test]
fn math_rotate_about_point() {
    // rotate a point about another point.
    // .-----------------------------> +X
    // |
    // |
    // |
    // |
    // |              orbit
    // |             (10,10)------>(15,10)  <= p
    // |                |
    // |                |
    // |                |
    // |                |
    // |                v (10,15)  <= p rotated 90 degrees about orbit.
    // |
    // |
    // v +Y
    // The orbit point is the location where p orbits about.
    let orbit_loc = Vector2::new(10.0, 10.0);

    // The point that orbits. It is located to the "right" of the orbit position by 5 units.
    let p = Vector2::new(orbit_loc.x + 5.0, orbit_loc.y);

    let mut tran = AffineTransform::new();
    tran.make_translate(orbit_loc.x, orbit_loc.y);
    let mut ntran = AffineTransform::new();
    ntran.make_translate(-orbit_loc.x, -orbit_loc.y);
    let mut rot = AffineTransform::new();
    rot.make_rotate(f64::to_radians(90.0));

    let mut at = AffineTransform::new();
    at.multiply(&ntran);
    at.multiply(&rot);
    at.multiply(&tran);

    let vr = AffineTransform::transform_vector(&p, &at);
    assert!(equal_abs(vr.x, 10.0));
    assert!(equal_abs(vr.y, 15.0));
}

#[test]
fn math_orbit_about_point() {
    let orbit_loc = Vector2::new(10.0, 10.0);

    // The point that orbits. It is located to the "right" of the orbit position by 5 units.
    let mut p = Vector2::new(orbit_loc.x + 5.0, orbit_loc.y);

    let mut orbiter = AffineTransform::new();
    AffineTransform::orbit_about_point(&mut p, &orbit_loc, 90.0, &mut orbiter);
    // println!("vr: {},{}", p.x, p.y);

    assert!(equal_abs(p.x, 10.0));
    assert!(equal_abs(p.y, 15.0));
}

#[test]
fn math_rotate_rectangle() {
    let mut r = Rectangle::from_points(Point::from_xy(-5.0, -5.0), Point::from_xy(5.0, 5.0));

    let mut at = AffineTransform::new();
    at.make_rotate(f64::to_radians(45.0));

    at.transform_rectangle(&mut r);
    println!("r: {:?}", r);

    assert!(r.min.x < 0.0);
    assert!(r.min.y < 0.0);
    assert!(equal_abs(r.min.x, -7.071068));
    assert!(equal_abs(r.min.y, -7.071068));
    assert!(equal_abs(r.max.x, 7.071068));
    assert!(equal_abs(r.max.y, 7.071068));
    assert!(equal_abs(r.width, 14.142136));
    assert!(equal_abs(r.height, 14.142136));
}
