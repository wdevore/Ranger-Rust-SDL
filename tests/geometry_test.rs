extern crate ranger;

use ranger::geometry::{point, rectangle};

#[test]
fn point_create() {
    let p = point::Point::new();
    assert_eq!(p.x, 0.0);
    assert_eq!(p.y, 0.0);
}

#[test]
fn rectangle_create() {
    let r = rectangle::Rectangle::new();
    assert_eq!(r.min.x, 0.0);
    assert_eq!(r.min.y, 0.0);
    assert_eq!(r.max.x, 1.0);
    assert_eq!(r.max.x, 1.0);

    let r = rectangle::Rectangle::from_points(point::Point::new(), point::Point::from_xy(5.0, 5.0));
    assert_eq!(r.min.x, 0.0);
    assert_eq!(r.min.y, 0.0);
    assert_eq!(r.max.x, 5.0);
    assert_eq!(r.max.x, 5.0);
}

#[test]
fn rectangle_width_height() {
    let r = rectangle::Rectangle::from_points(
        point::Point::from_xy(5.0, 5.0),
        point::Point::from_xy(15.0, 15.0),
    );
    assert_eq!(r.width, 10.0);
    assert_eq!(r.height, 10.0);
}

#[test]
fn rectangle_intersection_does() {
    let r1 =
        rectangle::Rectangle::from_points(point::Point::new(), point::Point::from_xy(10.0, 10.0));
    let r2 = rectangle::Rectangle::from_points(
        point::Point::from_xy(5.0, 5.0),
        point::Point::from_xy(15.0, 15.0),
    );

    let op_sect = r1.intersection(&r2);
    assert_ne!(op_sect, None);

    let sect = op_sect.unwrap();
    // println!("#################{:?} ", sect);

    assert_eq!(sect.min, point::Point::from_xy(5.0, 5.0));
    assert_eq!(sect.max, point::Point::from_xy(10.0, 10.0));
    assert_eq!(sect.width, 5.0);
    assert_eq!(sect.height, 5.0);
}

#[test]
fn rectangle_intersection_does_not() {
    let r1 =
        rectangle::Rectangle::from_points(point::Point::new(), point::Point::from_xy(10.0, 10.0));
    let r2 = rectangle::Rectangle::from_points(
        point::Point::from_xy(15.0, 15.0),
        point::Point::from_xy(25.0, 25.0),
    );

    let op_sect = r1.intersection(&r2);
    assert_eq!(op_sect, None);
}

#[test]
fn rectangle_intersects_does() {
    let r1 =
        rectangle::Rectangle::from_points(point::Point::new(), point::Point::from_xy(10.0, 10.0));
    let r2 = rectangle::Rectangle::from_points(
        point::Point::from_xy(5.0, 5.0),
        point::Point::from_xy(15.0, 15.0),
    );

    let intersects = r1.intersects(&r2);
    assert_eq!(intersects, true);
}

#[test]
fn rectangle_intersects_does_not() {
    let r1 =
        rectangle::Rectangle::from_points(point::Point::new(), point::Point::from_xy(10.0, 10.0));
    let r2 = rectangle::Rectangle::from_points(
        point::Point::from_xy(15.0, 15.0),
        point::Point::from_xy(25.0, 25.0),
    );

    let intersects = r1.intersects(&r2);
    assert_eq!(intersects, false);
}

#[test]
fn rectangle_intersection_upper_right() {
    let r1 = rectangle::Rectangle::from_points(
        point::Point::from_xy(10.0, 10.0),
        point::Point::from_xy(20.0, 20.0),
    );
    let r2 = rectangle::Rectangle::from_points(
        point::Point::from_xy(15.0, 5.0),
        point::Point::from_xy(25.0, 15.0),
    );

    let op_sect = r1.intersection(&r2);
    assert_ne!(op_sect, None);

    let sect = op_sect.unwrap();
    // println!("#################{:?} ", sect);

    assert_eq!(sect.min, point::Point::from_xy(15.0, 10.0));
    assert_eq!(sect.max, point::Point::from_xy(20.0, 15.0));
    assert_eq!(sect.width, 5.0);
    assert_eq!(sect.height, 5.0);
}

#[test]
fn rectangle_intersection_lower_left() {
    let r1 = rectangle::Rectangle::from_points(
        point::Point::from_xy(10.0, 10.0),
        point::Point::from_xy(20.0, 20.0),
    );
    let r2 = rectangle::Rectangle::from_points(
        point::Point::from_xy(5.0, 15.0),
        point::Point::from_xy(15.0, 25.0),
    );

    let op_sect = r1.intersection(&r2);
    assert_ne!(op_sect, None);

    let sect = op_sect.unwrap();
    // println!("#################{:?} ", sect);

    assert_eq!(sect.min, point::Point::from_xy(10.0, 15.0));
    assert_eq!(sect.max, point::Point::from_xy(15.0, 20.0));
    assert_eq!(sect.width, 5.0);
    assert_eq!(sect.height, 5.0);
}

#[test]
fn rectangle_union() {
    let r1 = rectangle::Rectangle::from_points(
        point::Point::from_xy(10.0, 10.0),
        point::Point::from_xy(20.0, 20.0),
    );
    let r2 = rectangle::Rectangle::from_points(
        point::Point::from_xy(15.0, 15.0),
        point::Point::from_xy(25.0, 25.0),
    );

    let rect = r1.union(&r2);

    // println!("#################{:?} ", rect);

    assert_eq!(rect.min, point::Point::from_xy(10.0, 15.0));
    assert_eq!(rect.max, point::Point::from_xy(25.0, 20.0));
    assert_eq!(rect.width, 15.0);
    assert_eq!(rect.height, 5.0);
}

#[test]
fn rectangle_bounding() {
    let r1 = rectangle::Rectangle::from_points(
        point::Point::from_xy(10.0, 10.0),
        point::Point::from_xy(20.0, 20.0),
    );
    let r2 = rectangle::Rectangle::from_points(
        point::Point::from_xy(15.0, 15.0),
        point::Point::from_xy(25.0, 25.0),
    );

    let rect = r1.bounding(&r2);

    // println!("#################{:?} ", rect);

    assert_eq!(rect.min, point::Point::from_xy(10.0, 10.0));
    assert_eq!(rect.max, point::Point::from_xy(25.0, 25.0));
    assert_eq!(rect.width, 15.0);
    assert_eq!(rect.height, 15.0);
}

#[test]
fn rectangle_contains_point() {
    let r1 = rectangle::Rectangle::from_points(
        point::Point::from_xy(10.0, 10.0),
        point::Point::from_xy(20.0, 20.0),
    );
    let p = point::Point::from_xy(15.0, 15.0);

    let contains = r1.contains_point(&p);

    // println!("#################{:?} ", rect);

    assert_eq!(contains, true);
}

#[test]
fn rectangle_not_contains_point() {
    let r1 = rectangle::Rectangle::from_points(
        point::Point::from_xy(10.0, 10.0),
        point::Point::from_xy(20.0, 20.0),
    );
    let mut p = point::Point::from_xy(35.0, 35.0);

    let contains = r1.contains_point(&p);

    // println!("#################{:?} ", rect);

    assert_eq!(contains, false);

    p.set_xy(0.0, 0.0);

    let contains = r1.contains_point(&p);

    // println!("#################{:?} ", rect);

    assert_eq!(contains, false);
}
