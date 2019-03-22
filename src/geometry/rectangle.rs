// use std::ops::Add;

use geometry::point::Point;

/// A two-dimensional axis-aligned rectangle.
///
/// X-axis directed towards the right
///
/// Y-Axis directed downward.
///
/// ```ignore
/// left(X)/Top(Y)
///       *------------.  --> X
///       |            |  |
///       |            |  v Y
///       |            |
///       |            |
///       .------------*
///              Right(X)/Bottom(Y)
/// ```
///
#[derive(Debug, PartialEq)]
pub struct Rectangle {
    /// Top left corner
    pub min: Point,
    /// Bottom-right corner
    pub max: Point,

    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            min: Point { x: 0.0, y: 0.0 },
            max: Point { x: 1.0, y: 1.0 },
            width: 1.0,
            height: 1.0,
        }
    }

    pub fn from_min_max(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self {
            min: Point::from_xy(min_x, min_y),
            max: Point::from_xy(max_x, max_y),
            width: max_x - min_x,
            height: max_y - min_y,
        }
    }

    pub fn from_points(min: Point, max: Point) -> Self {
        Self {
            min: min,
            max: max,
            width: max.x - min.x,
            height: max.y - min.y,
        }
    }

    pub fn set_min_max(&mut self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) {
        self.min.set_xy(min_x, min_y);
        self.max.set_xy(max_x, max_y);
        self.width = max_x - min_x;
        self.height = max_y - min_y;
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let x0 = f64::max(self.min.x, other.min.x);
        let x1 = f64::min(self.max.x, other.max.x);

        if x0 <= x1 {
            let y0 = f64::max(self.min.y, other.min.y);
            let y1 = f64::min(self.max.y, other.max.y);

            if y0 <= y1 {
                return Some(Self {
                    min: Point { x: x0, y: y0 },
                    max: Point { x: x1, y: y1 },
                    width: x1 - x0,
                    height: y1 - y0,
                });
            }
        }

        None
    }

    /// Returns `true` if `self` intersects `rectangle`
    pub fn intersects(&self, rectangle: &Self) -> bool {
        self.min.x <= rectangle.min.x + rectangle.width
            && rectangle.min.x <= self.min.x + self.width
            && self.max.y <= rectangle.max.y + rectangle.height
            && rectangle.max.y <= self.max.y + self.height
    }

    pub fn overlaps(&self, rectangle: &Self) -> bool {
        !((self.min.x + self.width < rectangle.min.x)
            || (rectangle.min.x + rectangle.width < self.min.x)
            || (self.max.y + self.height < rectangle.max.y)
            || (rectangle.max.y + rectangle.height < self.max.y))
    }

    /// Returns the smallest rectangle that contains both source rectangles.
    pub fn union(&self, rectangle: &Self) -> Self {
        let left = f64::min(self.min.x, rectangle.min.x);
        let top = f64::max(self.min.y, rectangle.min.y);
        let bottom = f64::min(self.max.y, rectangle.max.y);
        let right = f64::max(self.max.x, rectangle.max.x);

        Self {
            min: Point { x: left, y: top },
            max: Point {
                x: right,
                y: bottom,
            },
            width: right - left,
            height: bottom - top,
        }
    }

    /// Returns a new rectangle which completely contains `self` and `rectangle`.
    pub fn bounding(&self, rectangle: &Self) -> Self {
        let right = f64::max(self.max.x, rectangle.max.x);
        let bottom = f64::max(self.max.y, rectangle.max.y);
        let left = f64::min(self.min.x, rectangle.min.x);
        let top = f64::min(self.min.y, rectangle.min.y);

        Self {
            min: Point { x: left, y: top },
            max: Point {
                x: right,
                y: bottom,
            },
            width: right - left,
            height: bottom - top,
        }
    }

    pub fn contains_point(&self, p: &Point) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
}
