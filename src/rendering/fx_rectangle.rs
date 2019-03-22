extern crate sdl2;

use geometry::point::Point;
use rendering::fx_triangle::FXTriangle;
use rendering::render_context::Context;

pub struct FXRectangle {
    t1: FXTriangle,
    t2: FXTriangle,
}

// Coord system
//
// .-------> x    <== top
// |
// |
// |
// v y            <== bottom
impl FXRectangle {
    pub fn new() -> Self {
        Self {
            t1: FXTriangle::new(),
            t2: FXTriangle::new(),
        }
    }

    /// Points are specified in Clockwise order
    pub fn with_points(p0: Point, p1: Point, p2: Point, p3: Point) -> Self {
        Self {
            t1: FXTriangle::with_points(p0, p2, p3),
            t2: FXTriangle::with_points(p0, p1, p2),
        }
    }

    // top/left
    //     .-------------.  ==> +X
    //     |             |
    //     |             |
    //     |             |
    //     |             |
    //     .-------------. bottom/right
    //     |
    //     v
    //     +Y
    pub fn set(
        &mut self,
        top_left_x: f64,
        top_left_y: f64,
        bottom_right_x: f64,
        bottom_right_y: f64,
    ) {
        self.t1.set(
            top_left_x,
            bottom_right_y,
            bottom_right_x,
            top_left_y,
            top_left_x,
            top_left_y,
        );
        self.t2.set(
            top_left_x,
            bottom_right_y,
            bottom_right_x,
            bottom_right_y,
            bottom_right_x,
            top_left_y,
        );
    }

    pub fn draw(&self, context: &mut Context) {
        self.t1.draw(context);
        self.t2.draw(context);
    }
}
