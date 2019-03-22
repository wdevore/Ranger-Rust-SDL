use geometry::point::Point;
use rendering::render_context::Context;

pub struct NXTriangle {
    p0: Point,
    p1: Point,
    p2: Point,
}

impl NXTriangle {
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

    // Port of:
    // https://www.davrous.com/2013/06/21/tutorial-part-4-learning-how-to-write-a-3d-software-engine-in-c-ts-or-js-rasterization-z-buffering/

    // Naive slope form. Only use if you are having trouble with FXTriangles.
    // This fails between shared edges, it leaves a pixel gap.
    pub fn draw(&self, context: &Context) {
        // Sorting the points in order to always have this order on screen p1, p2 & p3
        // with p1 always up (thus having the Y the lowest possible to be near the top screen)
        // then p2 between p1 & p3
        let mut pt1 = self.p0;
        let mut pt2 = self.p1;
        let mut pt3 = self.p2;

        if pt1.y > pt2.y {
            let temp = pt2;
            pt2 = pt1;
            pt1 = temp;
        }

        if pt2.y > pt3.y {
            let temp = pt2;
            pt2 = pt3;
            pt3 = temp;
        }

        if pt1.y > pt2.y {
            let temp = pt2;
            pt2 = pt1;
            pt1 = temp;
        }

        // inverse slopes
        let dp1p2: f64;
        let dp1p3: f64;

        // Computing inverse slopes
        if pt2.y - pt1.y > 0.0 {
            dp1p2 = (pt2.x - pt1.x) / (pt2.y - pt1.y);
        } else {
            dp1p2 = 0.0;
        }

        if pt3.y - pt1.y > 0.0 {
            dp1p3 = (pt3.x - pt1.x) / (pt3.y - pt1.y);
        } else {
            dp1p3 = 0.0;
        }

        // First case where triangles are like this:
        if dp1p2 > dp1p3 {
            // P1
            // -
            // --
            // - -
            // -  -
            // -   - P2
            // -  -
            // - -
            // -
            // P3
            let mut y = pt1.y as i32;
            loop {
                if y > (pt3.y as i32) {
                    break;
                }
                if y < (pt2.y as i32) {
                    self.process_scan_line(y, &pt1, &pt3, &pt1, &pt2, context);
                } else {
                    self.process_scan_line(y, &pt1, &pt3, &pt2, &pt3, context);
                }
                y += 1;
            }
        } else {
            //       P1
            //        -
            //       --
            //      - -
            //     -  -
            // P2 -   -
            //     -  -
            //      - -
            //        -
            //       P3
            let mut y = pt1.y as i32;
            loop {
                if y > (pt3.y as i32) {
                    break;
                }
                if y < (pt2.y as i32) {
                    self.process_scan_line(y, &pt1, &pt2, &pt1, &pt3, context);
                } else {
                    self.process_scan_line(y, &pt2, &pt3, &pt1, &pt3, context);
                }
                y += 1;
            }
        }
    }

    // Clamping values to keep them between 0 and 1
    fn clamp(&self, value: f64, min: f64, max: f64) -> f64 {
        f64::max(min, f64::min(value, max))
    }

    // Interpolating the value between 2 vertices
    // min is the starting point, max the ending point
    // and gradient the % between the 2 points
    fn interpolate(&self, min: f64, max: f64, gradient: f64) -> f64 {
        return min + (max - min) * self.clamp(gradient, 0.0, 1.0);
    }

    fn process_scan_line(
        &self,
        y: i32,
        pa: &Point,
        pb: &Point,
        pc: &Point,
        pd: &Point,
        context: &Context,
    ) {
        // Thanks to current Y, we can compute the gradient to compute others values like
        // the starting X (sx) and ending X (ex) to draw between
        // if pa.Y == pb.Y or pc.Y == pd.Y, gradient is forced to 1
        let gradient1 = if pa.y != pb.y {
            ((y as f64) - pa.y) / (pb.y - pa.y)
        } else {
            1.0
        };

        let gradient2 = if pc.y != pd.y {
            ((y as f64) - pc.y) / (pd.y - pc.y)
        } else {
            1.0
        };

        let sx = self.interpolate(pa.x, pb.x, gradient1) as i32;
        let ex = self.interpolate(pc.x, pd.x, gradient2) as i32;

        // drawing a line from left (sx) to right (ex)
        let mut x = sx;
        loop {
            if x >= ex {
                break;
            }

            context.set_pixel(x, y);

            x += 1;
        }
    }
}
