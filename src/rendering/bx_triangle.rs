use geometry::point::Point;
use rendering::render_context::Context;

pub struct BXTriangle {
    p0: Point,
    p1: Point,
    p2: Point,
}

impl BXTriangle {
    pub fn new() -> Self {
        let p = Point::from_xy(0.0, 0.0);

        Self {
            p0: p,
            p1: p,
            p2: p,
        }
    }

    pub fn with_points(p0: Point, p1: Point, p2: Point) -> Self {
        Self { p0, p1, p2 }
    }

    pub fn set(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.p0.set_xy(x0, y0);
        self.p1.set_xy(x1, y1);
        self.p2.set_xy(x2, y2);
    }

    pub fn draw(&self, context: &Context) {
        let mut p0 = self.p0;
        let mut p1 = self.p1;
        let mut p2 = self.p2;

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

        let total_height = (p2.y - p0.y) as i32;

        for i in 0..total_height {
            let second_half = i as u32 > (p1.y - p0.y) as u32 || p1.y == p0.y;
            let segment_height = if second_half {
                p2.y - p1.y
            } else {
                p1.y - p0.y
            };

            let alpha = i as f64 / total_height as f64;
            let sub = if second_half { p1.y - p0.y } else { 0.0 };
            let beta = (i as f64 - sub) / segment_height;
            // be careful: with above conditions no division by zero here
            let a1 = p2 - p0;
            let a2 = Point::from_xy(a1.x * alpha, a1.y * alpha);
            let mut a = p0 + a2;

            let b1 = p2 - p1;
            let b2 = p1 - p0;
            let b3 = Point::from_xy(b1.x * beta, b1.y * beta);
            let b4 = Point::from_xy(b2.x * beta, b2.y * beta);
            let mut b = if second_half { p1 + b3 } else { p0 + b4 };
            if a.x > b.x {
                let t = b;
                b = a;
                a = t;
            }

            for j in (a.x as i32)..(b.x as i32) {
                context.set_pixel(j, (p0.y as i32) + i);
            }
        }
    }
}
