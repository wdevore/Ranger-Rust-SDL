// http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html

use geometry::aabb::AABBox;
use geometry::point::Point;
use rendering::render_context::Context;

pub struct SXTriangle {
    p0: Point,
    p1: Point,
    p2: Point,
    aabb: AABBox,
    orientation: u32,
}

impl SXTriangle {
    pub fn new() -> Self {
        let p = Point::from_xy(0.0, 0.0);

        Self {
            p0: p,
            p1: p,
            p2: p,
            aabb: AABBox::new(),
            orientation: 1, // default to CCW
        }
    }

    pub fn with_points(p0: Point, p1: Point, p2: Point) -> Self {
        let mut ab = AABBox::new();
        ab.set(&p0, &p1, &p2);
        Self {
            p0,
            p1,
            p2,
            aabb: ab,
            orientation: orientation(&p0, &p1, &p2),
        }
    }

    pub fn set(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.p0.set_xy(f64::trunc(x0), f64::trunc(y0));
        self.p1.set_xy(f64::trunc(x1), f64::trunc(y1));
        self.p2.set_xy(f64::trunc(x2), f64::trunc(y2));

        // find triangle's AABB.
        self.aabb.set(&self.p0, &self.p1, &self.p2);
        self.orientation = orientation(&self.p0, &self.p1, &self.p2);
        // println!("orie: {}", self.orientation);
    }

    pub fn draw(&self, context: &Context) {
        let mut p0 = self.p0;
        let mut p1 = self.p1;
        let mut p2 = self.p2;
        let mut intersect = Point::new();

        // we dont care about degenerate triangles
        if p0.y == p1.y && p0.y == p2.y {
            return;
        }

        if p0.y > p1.y {
            let t = p1;
            p1 = p0;
            p0 = t;
        }

        if p0.y > p2.y {
            let t = p2;
            p2 = p0;
            p0 = t;
        }

        if p1.y > p2.y {
            let t = p2;
            p2 = p1;
            p1 = t;
        }

        // here we know that p0.y <= p1.y <= p2.y
        // check for trivial case of bottom-flat triangle
        if p1.y == p2.y {
            fill_bottom_flat_triangle(&p0, &p1, &p2, context);
        }
        // check for trivial case of top-flat triangle
        else if p0.y == p1.y {
            fill_top_flat_triangle(&p0, &p1, &p2, context);
        } else {
            // general case - split the triangle in a topflat and bottom-flat one
            intersect.set_xy(
                f64::trunc(p0.x + ((p1.y - p0.y) / (p2.y - p0.y)) * (p2.x - p0.x)),
                p1.y,
            );
            // (vt1.x + ((vt2.y - vt1.y) / (vt3.y - vt1.y)) * (vt3.x - vt1.x)), vt2.y);

            fill_bottom_flat_triangle(&p0, &p1, &intersect, context);
            fill_top_flat_triangle(&p1, &intersect, &p2, context);
        }
    }
}

fn fill_bottom_flat_triangle(p0: &Point, p1: &Point, p2: &Point, context: &Context) {
    let slope1 = (p1.x - p0.x) / (p1.y - p0.y);
    let slope2 = (p2.x - p0.x) / (p2.y - p0.y);
    let mut x1 = p0.x; // + 0.5;
    let mut x2 = p0.x; // + 0.5;

    for scanline_y in (p0.y as i32)..((p1.y + 1.0) as i32) {
        context.draw_horz_line(x1 as i32, x2 as i32, scanline_y);
        x1 += slope1;
        x2 += slope2;
    }
}

fn fill_top_flat_triangle(p0: &Point, p1: &Point, p2: &Point, context: &Context) {
    let slope1 = (p2.x - p0.x) / (p2.y - p0.y);
    let slope2 = (p2.x - p1.x) / (p2.y - p1.y);
    let mut x1 = p2.x; // + 0.5;
    let mut x2 = p2.x; // + 0.5;

    for scanline_y in ((p0.y as i32)..((p2.y + 1.0) as i32)).rev() {
        context.draw_horz_line(x1 as i32, x2 as i32, scanline_y);
        x1 -= slope1;
        x2 -= slope2;
    }
}

/// Find orientation of ordered triplet (p1, p2, p3).
/// The function returns the following values
///
/// # returns
///
/// * 0 --> Colinear
/// * 1 --> Counterclockwise
/// * 2 --> Clockwise
fn orientation(p0: &Point, p1: &Point, p2: &Point) -> u32 {
    // Cross product
    let val = (p1.y - p0.y) * (p2.x - p1.x) - (p1.x - p0.x) * (p2.y - p1.y);

    if val == 0.0 {
        // TODO should use epsilon comparison.
        return 0; // colinear
    }

    // clock or counterclock wise
    if val > 0.0 {
        1
    } else {
        2
    }
}
