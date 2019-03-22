use std::cell::RefCell;
use std::rc::Rc;

use geometry::point::Point;
use geometry::rectangle::Rectangle;
use math::vector2::Vector2;

pub type RAffineTransform = Rc<RefCell<AffineTransform>>;

// A minified affine transform.
// Column major
//     x'   |a c tx|   |x|
//     y' = |b d ty| x |y|                  <=== Post multiply
//     1    |0 0  1|   |1|
// or
// Row major
//                           |a  b   0|
//     |x' y' 1| = |x y 1| x |c  d   0|     <=== Pre multiply
//                           |tx ty  1|
//
#[derive(Debug, Clone, Copy)]
pub struct AffineTransform {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}

impl AffineTransform {
    pub fn new() -> Self {
        AffineTransform::from_components(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    }

    pub fn from_transform(t: &AffineTransform) -> Self {
        AffineTransform::from_components(t.a, t.b, t.c, t.d, t.tx, t.ty)
    }

    pub fn from_components(a: f64, b: f64, c: f64, d: f64, tx: f64, ty: f64) -> Self {
        Self {
            a: a,
            b: b,
            c: c,
            d: d,
            tx: tx,
            ty: ty,
        }
    }

    pub fn to_identity(&mut self) {
        self.a = 1.0;
        self.b = 0.0;
        self.c = 0.0;
        self.d = 1.0;
        self.tx = 0.0;
        self.ty = 0.0;
    }

    pub fn set(&mut self, a: f64, b: f64, c: f64, d: f64, tx: f64, ty: f64) {
        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;
        self.tx = tx;
        self.ty = ty;
    }

    pub fn mul_vector(&self, v: &mut Vector2) {
        v.x = (self.a * v.x) + (self.c * v.y) + self.tx;
        v.y = (self.b * v.x) + (self.d * v.y) + self.ty;
    }

    pub fn mul_components(&self, x: f64, y: f64) -> (f64, f64) {
        (
            (self.a * x) + (self.c * y) + self.tx,
            (self.b * x) + (self.d * y) + self.ty,
        )
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        self.tx = (self.a * x) + (self.c * y) + self.tx;
        self.ty = (self.b * x) + (self.d * y) + self.ty;
    }

    pub fn set_translate(&mut self, tx: f64, ty: f64) {
        self.tx = tx;
        self.ty = ty;
    }

    pub fn make_translate(&mut self, tx: f64, ty: f64) {
        self.set(1.0, 0.0, 0.0, 1.0, tx, ty);
    }

    pub fn keep_translate(&mut self) {
        let tx = self.tx;
        let ty = self.ty;
        self.set(1.0, 0.0, 0.0, 1.0, tx, ty);
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
        self.a *= sx;
        self.b *= sx;
        self.c *= sy;
        self.d *= sy;
    }

    pub fn set_scale(&mut self, sx: f64, sy: f64) {
        self.set(sx, 0.0, 0.0, sy, 0.0, 0.0);
    }

    // Concatinate a rotation (radians) onto this transform.
    //
    // Rotation is just a matter of perspective. A CW rotation can be seen as
    // CCW depending on what you are talking about rotating. For example,
    // if the coordinate system is thought as rotating CCW then objects are
    // seen as rotating CW, and that is what the 2x2 matrix below represents.
    //
    // It is also the frame of reference we use. In this library +Y axis is downward
    //     |cos  -sin|   object appears to rotate CW.
    //     |sin   cos|
    //
    // In the matrix below the object appears to rotate CCW.
    //     |cos  sin|
    //     |-sin cos|
    //
    //     |a  c|    |cos  -sin|
    //     |b  d|  x |sin   cos|
    //
    pub fn rotate(&mut self, radians: f64) {
        let sin = f64::sin(radians);
        let cos = f64::cos(radians);
        let _a = self.a;
        let _b = self.b;
        let _c = self.c;
        let _d = self.d;

        // |a1 c1|   |a2 c2|   |a1a2 + a1b2, a1c2 + c1d2|
        // |b1 d1| x |b2 d2| = |b1a2 + d1b2, b1c2 + d1d2|
        //
        // |_a, _c|   |cos, -sin|   |_acos + _csin, _a(-sin) + _ccos|
        // |_b, _d| x |sin,  cos| = |_bcos + _dsin, _b(-sin) + _dcos|
        self.a = _a * cos + _c * sin;
        self.b = _b * cos + _d * sin;
        self.c = _c * cos - _a * sin;
        self.d = _d * cos - _b * sin;
    }

    // Assuming that +Y axis is down then:
    //     a c   --> cos  sin
    //     b d   --> -sin cos
    // Yields a CW rotation for a positive angle.
    pub fn set_rotate(&mut self, radians: f64) {
        let sin = f64::sin(radians);
        let cos = f64::cos(radians);
        self.a = cos;
        self.b = sin;
        self.c = -sin;
        self.d = cos;
    }

    pub fn make_rotate(&mut self, radians: f64) {
        let sin = f64::sin(radians);
        let cos = f64::cos(radians);
        self.a = cos;
        self.b = sin;
        self.c = -sin;
        self.d = cos;
        self.tx = 0.0;
        self.ty = 0.0;
    }

    // pre-multiply
    pub fn multiply(&mut self, at: &AffineTransform) {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = self.d;
        let tx = self.tx;
        let ty = self.ty;

        self.a = a * at.a + b * at.c;
        self.b = a * at.b + b * at.d;
        self.c = c * at.a + d * at.c;
        self.d = c * at.b + d * at.d;
        self.tx = (tx * at.a) + (ty * at.c) + at.tx;
        self.ty = (tx * at.b) + (ty * at.d) + at.ty;
    }

    pub fn invert(&mut self) {
        let determinant = 1.0 / (self.a * self.d - self.b * self.c);
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = self.d;
        let tx = self.tx;
        let ty = self.ty;

        self.a = determinant * d;
        self.b = -determinant * b;
        self.c = -determinant * c;
        self.d = determinant * a;
        self.tx = determinant * (c * ty - d * tx);
        self.ty = determinant * (b * tx - a * ty);
    }

    pub fn inverse(&self) -> AffineTransform {
        let mut inv = AffineTransform::from_transform(self);
        inv.invert();
        inv
    }

    // Converts either from or to pre or post multiplication.
    //     a c
    //     b d
    // to
    //     a b
    //     c d
    pub fn transpose(&mut self) {
        let _c = self.c;
        self.c = self.b;
        self.b = _c;
    }

    // Reliable as long as the transform does not have a Skew present.
    // http://stackoverflow.com/tags/affinetransform/info
    pub fn extract_rotation(&self) -> f64 {
        f64::atan2(self.c, self.d)
    }

    pub fn extract_scale_x(&self) -> f64 {
        f64::sqrt(self.a * self.a + self.b * self.b)
    }

    pub fn extract_scale_y(&self) -> f64 {
        f64::sqrt(self.d * self.d + self.c * self.c)
    }

    // Produces an Axis aligned bounding rectangle/box (AABB)
    pub fn transform_rectangle(&self, rect: &mut Rectangle) {
        // Each corner of the rectangle is transformed. The new positions
        // are used to establish the new rectangle.

        // left(X)/Top(Y)     right/top
        //       *------------.  --> +X
        //       |            |  |
        //       |            |  v +Y
        //       |            |
        //       |            |
        //       .------------*
        //  left/bottom      Right(X)/Bottom(Y)

        let left_top = AffineTransform::transform_components(rect.min.x, rect.min.y, self);
        let right_top = AffineTransform::transform_components(rect.max.x, rect.min.y, self);
        let right_bottom = AffineTransform::transform_components(rect.max.x, rect.max.y, self);
        let left_bottom = AffineTransform::transform_components(rect.min.x, rect.max.y, self);

        // Calc the new corners
        let min_x = f64::min(
            f64::min(left_top.x, right_top.x),
            f64::min(left_bottom.x, right_bottom.x),
        );

        let max_x = f64::max(
            f64::max(left_top.x, right_top.x),
            f64::max(left_bottom.x, right_bottom.x),
        );

        let min_y = f64::min(
            f64::min(left_top.y, right_top.y),
            f64::min(left_bottom.y, right_bottom.y),
        );

        let max_y = f64::max(
            f64::max(left_top.y, right_top.y),
            f64::max(left_bottom.y, right_bottom.y),
        );

        rect.set_min_max(min_x, min_y, max_x, max_y);
    }

    // #########################################################################
    // Associated functions
    // #########################################################################
    // #[inline(always)]
    // fn transform(x: &mut f64, y: &mut f64, a: f64, b: f64, c: f64, d: f64, tx: f64, ty: f64) {
    //     *x = (a * *x) + (c * *y) + tx;
    //     *y = (b * *x) + (d * *y) + ty;
    // }

    /// A handy method for orbiting a point around a central point.
    ///
    /// #Arguments
    ///
    /// * point - The point that orbits relative to the center
    /// * orbit_about - The center where `point` orbits about
    /// * degrees - How far around the center relative to the +X axis. Rotation is CW.
    /// * at - A working/scratch transform that will be configured and applied to `point`
    ///
    /// ```ignore
    /// (0,0)
    /// .-----------------------------> +X
    /// |
    /// |
    /// |        orbit
    /// |       (10,10)------>(15,10)  <= p
    /// |          |
    /// |          |
    /// |          |
    /// |          |
    /// |          v (10,15)  <= p rotated 90
    /// |                        degrees about orbit.
    /// |           
    /// |
    /// v +Y
    /// ```
    ///
    pub fn orbit_about_point(
        point: &mut Vector2,
        orbit_about: &Vector2,
        degrees: f64,
        at: &mut AffineTransform,
    ) {
        // Below uses post-multiply methods verses the pre-multiply methods.
        at.set_translate(orbit_about.x, orbit_about.y);
        at.rotate(f64::to_radians(degrees)); // <-- post-multiply methods
        at.translate(-orbit_about.x, -orbit_about.y);
        AffineTransform::transform_to_vector(point.x, point.y, point, at);

        // -------------------------------------------
        // Below is the pre-multiply approach:
        // -------------------------------------------
        // let mut tran = AffineTransform::new();
        // tran.set_translate(orbit_about.x, orbit_about.y);
        // let mut ntran = AffineTransform::new();
        // ntran.set_translate(-orbit_about.x, -orbit_about.y);
        // let mut rot = AffineTransform::new();
        // rot.set_rotate(f64::to_radians(degrees));

        // let mut at = AffineTransform::new();
        // at.multiply(&ntran);  // <-- pre-multiply methods
        // at.multiply(&rot);
        // at.multiply(&tran);

        // AffineTransform::transform_to_vector(point.x, point.y, point, &at);
    }

    pub fn transform_vector(v: &Vector2, at: &AffineTransform) -> Vector2 {
        AffineTransform::transform_components(v.x, v.y, at)
    }

    pub fn transform_to_vector(x: f64, y: f64, out: &mut Vector2, at: &AffineTransform) {
        out.x = (at.a * x) + (at.c * y) + at.tx;
        out.y = (at.b * x) + (at.d * y) + at.ty;
    }

    pub fn transform_to_point(p: &Point, outp: &mut Point, at: &AffineTransform) {
        outp.x = (at.a * p.x) + (at.c * p.y) + at.tx;
        outp.y = (at.b * p.x) + (at.d * p.y) + at.ty;
    }

    pub fn transform_components(x: f64, y: f64, at: &AffineTransform) -> Vector2 {
        Vector2::new(
            (at.a * x) + (at.c * y) + at.tx,
            (at.b * x) + (at.d * y) + at.ty,
        )
    }

    ///
    /// ```ignore
    /// Applies: out = m * n
    /// ```
    pub fn multiply_mn(m: &AffineTransform, n: &AffineTransform, out: &mut AffineTransform) {
        out.set(
            m.a * n.a + m.b * n.c,
            m.a * n.b + m.b * n.d,
            m.c * n.a + m.d * n.c,
            m.c * n.b + m.d * n.d,
            (m.tx * n.a) + (m.ty * n.c) + n.tx,
            (m.tx * n.b) + (m.ty * n.d) + n.ty,
        );
    }

    pub fn invert_mo(m: &AffineTransform, out: &mut AffineTransform) {
        let determinant = 1.0 / (m.a * m.d - m.b * m.c);

        out.set(
            determinant * m.d,
            -determinant * m.b,
            -determinant * m.c,
            determinant * m.a,
            determinant * (m.c * m.ty - m.d * m.tx),
            determinant * (m.b * m.tx - m.a * m.ty),
        );
    }
}
