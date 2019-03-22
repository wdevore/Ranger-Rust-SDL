use geometry::point::Point;
use std::f64;

#[derive(Debug, PartialEq)]
pub struct AABBox {
    /// Top left corner
    pub min: Point,
    /// Bottom-right corner
    pub max: Point,
}

impl AABBox {
    pub fn new() -> Self {
        Self {
            min: Point::new(),
            max: Point::new(),
        }
    }

    pub fn set(&mut self, p0: &Point, p1: &Point, p2: &Point) {
        self.min.set_xy(
            f64::min(p0.x, f64::min(p1.x, p2.x)),
            f64::min(p0.y, f64::min(p1.y, p2.y)),
        );
        self.max.set_xy(
            f64::max(p0.x, f64::max(p1.x, p2.x)),
            f64::max(p0.y, f64::max(p1.y, p2.y)),
        );
    }

    pub fn set_from_vertices(&mut self, vertices: &Vec<Point>) {
        let mut minx = f64::MAX;
        let mut miny = f64::MAX;
        let mut maxx = f64::MIN;
        let mut maxy = f64::MIN;

        for v in vertices.iter() {
            minx = f64::min(minx, v.x);
            maxx = f64::max(maxx, v.x);
            miny = f64::min(miny, v.y);
            maxy = f64::max(maxy, v.y);
        }

        self.min.set_xy(minx, miny);
        self.max.set_xy(maxx, maxy);
    }
}
