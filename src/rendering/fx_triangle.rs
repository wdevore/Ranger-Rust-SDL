extern crate sdl2;

use std::cell::RefCell;

use geometry::point::Point;
use rendering::fx_edge::FXEdge;
use rendering::render_context::Context;

// This is the fastest rasterizer of the bunch.

// A port of Chris Hecker's texture rasterizers with the
// the texture part removed, thus making this a simple
// flat triangle rasterizer.
// http://chrishecker.com/Miscellaneous_Technical_Articles

type RfFXEdge = RefCell<FXEdge>;

// -----------------------------------------------------
// Triangle
// -----------------------------------------------------
pub struct FXTriangle {
    p0: Point,
    p1: Point,
    p2: Point,

    // By defining the edges as RefCells the draw() method can be defined
    // as immutable.
    top_to_bottom: RfFXEdge,
    top_to_middle: RfFXEdge,
    middle_to_bottom: RfFXEdge,

    /// true = clockwise, false = counter-clockwise
    clockwise: bool,
}

// Coord system
//
// .-------> x    <--- top
// |
// |
// |
// v y            <-- bottom
impl FXTriangle {
    pub fn new() -> Self {
        let p = Point::from_xy(0.0, 0.0);

        Self {
            p0: p,
            p1: p,
            p2: p,
            top_to_bottom: RefCell::new(FXEdge::new()),
            top_to_middle: RefCell::new(FXEdge::new()),
            middle_to_bottom: RefCell::new(FXEdge::new()),
            clockwise: false,
        }
    }

    /// Points are specified in Clockwise order
    pub fn with_points(p0: Point, p1: Point, p2: Point) -> Self {
        Self {
            p0: p0,
            p1: p1,
            p2: p2,
            top_to_bottom: RefCell::new(FXEdge::new()),
            top_to_middle: RefCell::new(FXEdge::new()),
            middle_to_bottom: RefCell::new(FXEdge::new()),
            clockwise: if orientation(&p0, &p1, &p2) == 1 {
                true
            } else {
                false
            },
        }
    }

    pub fn set(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.p0.set_xy(f64::trunc(x0), f64::trunc(y0));
        self.p1.set_xy(f64::trunc(x1), f64::trunc(y1));
        self.p2.set_xy(f64::trunc(x2), f64::trunc(y2));
        self.clockwise = if orientation(&self.p0, &self.p1, &self.p2) == 1 {
            true
        } else {
            false
        };
    }

    pub fn set_clockwise(&mut self) {
        self.clockwise = true;
    }

    pub fn draw(&self, context: &Context) {
        let top: &Point;
        let middle: &Point;
        let bottom: &Point;
        let middle_for_compare: u8;
        let bottom_for_compare: u8;
        let y0 = self.p0.y;
        let y1 = self.p1.y;
        let y2 = self.p2.y;

        // sort vertices in y
        if y0 < y1 {
            if y2 < y0 {
                top = &self.p2;
                middle = &self.p0;
                bottom = &self.p1;
                middle_for_compare = 0;
                bottom_for_compare = 1;
            } else {
                top = &self.p0;
                if y1 < y2 {
                    middle = &self.p1;
                    bottom = &self.p2;
                    middle_for_compare = 1;
                    bottom_for_compare = 2;
                } else {
                    middle = &self.p2;
                    bottom = &self.p1;
                    middle_for_compare = 2;
                    bottom_for_compare = 1;
                }
            }
        } else {
            if y2 < y1 {
                top = &self.p2;
                middle = &self.p1;
                bottom = &self.p0;
                middle_for_compare = 1;
                bottom_for_compare = 0;
            } else {
                top = &self.p1;
                if y0 < y2 {
                    middle = &self.p0;
                    bottom = &self.p2;
                    middle_for_compare = 3;
                    bottom_for_compare = 2;
                } else {
                    middle = &self.p2;
                    bottom = &self.p0;
                    middle_for_compare = 2;
                    bottom_for_compare = 3;
                }
            }
        }

        self.top_to_bottom.borrow_mut().set(top, bottom);
        self.top_to_middle.borrow_mut().set(top, middle);
        self.middle_to_bottom.borrow_mut().set(middle, bottom);

        let middle_is_left: bool;
        let mut height = self.top_to_middle.borrow().height;

        // the triangle is clockwise, so if bottom > middle then middle is right
        // the triangle is counter-clockwise, so if bottom > middle then middle is left
        if bottom_for_compare > middle_for_compare {
            middle_is_left = false;
            for _ in 0..height {
                if self.clockwise {
                    let ttb = self.top_to_bottom.borrow();
                    context.draw_horz_line(
                        ttb.x as i32,
                        self.top_to_middle.borrow().x as i32,
                        ttb.y as i32,
                    );
                } else {
                    let ttm = self.top_to_middle.borrow();
                    context.draw_horz_line(
                        ttm.x as i32,
                        self.top_to_bottom.borrow().x as i32,
                        ttm.y as i32,
                    );
                }

                self.top_to_bottom.borrow_mut().step();
                self.top_to_middle.borrow_mut().step();
            }
        } else {
            middle_is_left = true;
            for _ in 0..height {
                if self.clockwise {
                    let ttm = self.top_to_middle.borrow();
                    context.draw_horz_line(
                        ttm.x as i32,
                        self.top_to_bottom.borrow().x as i32,
                        ttm.y as i32,
                    );
                } else {
                    let ttb = self.top_to_bottom.borrow();
                    context.draw_horz_line(
                        ttb.x as i32,
                        self.top_to_middle.borrow().x as i32,
                        ttb.y as i32,
                    );
                }

                self.top_to_bottom.borrow_mut().step();
                self.top_to_middle.borrow_mut().step();
            }
        }

        height = self.middle_to_bottom.borrow().height;

        if middle_is_left {
            for _ in 0..height {
                if self.clockwise {
                    let mtb = self.middle_to_bottom.borrow();
                    context.draw_horz_line(
                        mtb.x as i32,
                        self.top_to_bottom.borrow().x as i32,
                        mtb.y as i32,
                    );
                } else {
                    let ttb = self.top_to_bottom.borrow();
                    context.draw_horz_line(
                        ttb.x as i32,
                        self.middle_to_bottom.borrow().x as i32,
                        ttb.y as i32,
                    );
                }

                self.middle_to_bottom.borrow_mut().step();
                self.top_to_bottom.borrow_mut().step();
            }
        } else {
            for _ in 0..height {
                if self.clockwise {
                    let ttb = self.top_to_bottom.borrow();
                    context.draw_horz_line(
                        ttb.x as i32,
                        self.middle_to_bottom.borrow().x as i32,
                        ttb.y as i32,
                    );
                } else {
                    let mtb = self.middle_to_bottom.borrow();
                    context.draw_horz_line(
                        mtb.x as i32,
                        self.top_to_bottom.borrow().x as i32,
                        mtb.y as i32,
                    );
                }

                self.top_to_bottom.borrow_mut().step();
                self.middle_to_bottom.borrow_mut().step();
            }
        }
    }

    // #[inline]
    // fn draw_scanline(&self, l_x: i64, r_x: i64, l_y: i64, context: &mut Context) {
    //     context.draw_horz_line(l_x, r_x, l_y);
    //     // let mut x_start = l_x;
    //     // let width = r_x - x_start;
    //     // for _ in 0..width {
    //     //     context.set_pixel(x_start, l_y);
    //     //     x_start += 1; // move to next pixel
    //     // }
    // }
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
        2
    } else {
        1
    }
}
