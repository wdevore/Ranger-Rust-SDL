use geometry::aabb::AABBox;
use geometry::point::Point;
use rendering::render_context::Context;

// Barycentric algorithm

// This rasterizer is based mostly on these article
// https://erkaman.github.io/posts/fast_triangle_rasterization.html
// https://github.com/Erkaman/sse-avx-rasterization/blob/master/main.cpp
//
// with the help of these too:
// https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/rasterization-stage
// http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
// http://www.inf.ed.ac.uk/teaching/courses/cg/lectures/cg7_2016.pdf

pub struct EXTriangle {
    p0: Point,
    p1: Point,
    p2: Point,
    aabb: AABBox,
    orientation: u32,
}

impl EXTriangle {
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
        self.p0.set_xy(x0, y0);
        self.p1.set_xy(x1, y1);
        self.p2.set_xy(x2, y2);

        // find triangle's AABB.
        self.aabb.set(&self.p0, &self.p1, &self.p2);
        self.orientation = orientation(&self.p0, &self.p1, &self.p2);
        // println!("orie: {}", self.orientation);
    }

    pub fn draw(&self, context: &Context) {
        let mut p = Point::new();
        let mut w0: f64;
        let mut w1: f64;
        let mut w2: f64;
        for yc in (self.aabb.min.y as i32)..((self.aabb.max.y + 1.0) as i32) {
            for xr in (self.aabb.min.x as i32)..((self.aabb.max.x + 1.0) as i32) {
                p.set_xy(xr as f64, yc as f64);
                if self.orientation == 1 {
                    w0 = edge_function(&self.p1, &self.p2, &p);
                    w1 = edge_function(&self.p2, &self.p0, &p);
                    w2 = edge_function(&self.p0, &self.p1, &p);
                } else {
                    w0 = edge_function(&self.p2, &self.p1, &p);
                    w1 = edge_function(&self.p1, &self.p0, &p);
                    w2 = edge_function(&self.p0, &self.p2, &p);
                }
                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    context.set_pixel(xr, yc);
                }
            }
        }
    }
}

#[inline(always)]
fn edge_function(a: &Point, b: &Point, c: &Point) -> f64 {
    // we are doing the reversed edge test, compared to the article.
    // we need to do it in this way, since our coordinate system has the origin in the top-left corner.
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
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

// fn rounddown_aligned(i: u32, align: u32) -> u32 {
//     (f64::floor((i as f64) / (align as f64)) as u32) * align
// }

// fn roundup_aligned(i: u32, align: u32) -> u32 {
//     (f64::ceil((i as f64) / (align as f64)) as u32) * align
// }

// fn clamp(f: f64, min: f64, max: f64) -> f64 {
//     if f < min {
//         return min;
//     } else if f > max {
//         return max;
//     }

//     f
// }
