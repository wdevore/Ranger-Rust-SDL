extern crate font8x8;
extern crate sdl2;

use std::cell::RefCell;
// use std::rc::Rc;

use self::font8x8::{UnicodeFonts, BASIC_FONTS};

use self::sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::BlendMode,
};

use geometry::{aabb::AABBox, point::Point as RPoint};
use math::affine_transform::AffineTransform;
// use rendering::color::Color as RColor;
// use rendering::bx_triangle::BXTriangle;
use rendering::color::Palette;
// use rendering::ex_triangle::EXTriangle;
use rendering::fx_triangle::FXTriangle;
// use rendering::nx_triangle::NXTriangle;
// use rendering::sx_triangle::SXTriangle;
use world::{RCCanvas, WorldProperties};

const STATE_STACK_DEPTH: usize = 100;

pub enum RenderStyle {
    FILLED,
    OUTLINE,
    BOTH,
}

#[derive(Copy, Clone, Debug)]
struct State {
    clear_color: Color,
    draw_color: Color,
    current: AffineTransform,
    filtered: AffineTransform,
}

pub struct Context {
    world_properties: WorldProperties,

    // State management
    state: Vec<State>,
    stack_top: usize,

    clear_color: Color,
    draw_color: Color,

    // SDL canvas target.
    canvas: RCCanvas,

    current_aft: AffineTransform,
    post: AffineTransform,

    // view space to device-space projection
    view_space: AffineTransform,

    // Triangle rasterizers
    fx_rasterizer: RefCell<FXTriangle>,
    // bx_rasterizer: RefCell<BXTriangle>,
    // nx_rasterizer: RefCell<NXTriangle>,
    // ex_rasterizer: RefCell<EXTriangle>,
    // sx_rasterizer: RefCell<SXTriangle>,
}

// SceneManager creates the Context.
impl Context {
    pub fn new(canvas: RCCanvas) -> Self {
        Self {
            world_properties: WorldProperties::new(),
            state: Vec::with_capacity(STATE_STACK_DEPTH),
            stack_top: 0,
            clear_color: Color::RGB(32, 32, 32),
            draw_color: Color::RGB(0, 0, 0),
            canvas: canvas,
            current_aft: AffineTransform::new(),
            post: AffineTransform::new(),
            view_space: AffineTransform::new(),
            fx_rasterizer: RefCell::new(FXTriangle::new()),
            // bx_rasterizer: RefCell::new(BXTriangle::new()),
            // nx_rasterizer: RefCell::new(NXTriangle::new()),
            // ex_rasterizer: RefCell::new(EXTriangle::new()),
            // sx_rasterizer: RefCell::new(SXTriangle::new()),
        }
    }

    pub fn initialize(&mut self, wp: &WorldProperties) {
        self.world_properties.set(wp);
        self.canvas.borrow_mut().set_blend_mode(BlendMode::Blend);

        let copy = State {
            clear_color: Color::RGB(0, 0, 0),
            draw_color: Color::RGB(0, 0, 0),
            current: AffineTransform::new(),
            filtered: AffineTransform::new(),
        };

        for _ in 0..STATE_STACK_DEPTH {
            self.state.push(copy);
        }

        self.set_view_space();
    }

    pub fn set_view_space(&mut self) {
        // Apply view-space matrix
        let mut cent = AffineTransform::new();

        // What separates world from view is the ratio between the device (aka window)
        // and an optional centering translation.
        let width_ratio =
            self.world_properties.window_width as f64 / self.world_properties.view_width;
        let height_ratio =
            self.world_properties.window_height as f64 / self.world_properties.view_height;

        if self.world_properties.view_centered {
            cent.make_translate(
                self.world_properties.window_width as f64 / 2.0,
                self.world_properties.window_height as f64 / 2.0,
            );
        }
        cent.scale(width_ratio, height_ratio);
        self.view_space = cent;

        self.apply(&cent);
    }

    pub fn get_view_space(&self) -> &AffineTransform {
        &self.view_space
    }

    // ----------------------------------------------------------
    // Color
    // ----------------------------------------------------------
    pub fn set_clear_color(&mut self, color: Palette) {
        self.clear_color = Color::RGB(color.r, color.g, color.b);
    }

    pub fn set_draw_color(&mut self, color: &Palette) {
        self.draw_color = Color::RGBA(color.r, color.g, color.b, color.a);
        self.canvas.borrow_mut().set_draw_color(self.draw_color);
    }

    /// Clears the background canvas
    pub fn clear(&self) {
        // {
        //     let mut can = self.canvas.borrow_mut();
        //     can.set_draw_color(self.clear_color);
        //     can.clear();
        // }

        // Draw checkerboard as an clear indicator for debugging
        let w = self.world_properties.window_width as i32;
        let h = self.world_properties.window_height as i32;
        let mut flip = false;
        let size = 200i32;
        let mut col = 0i32;
        let mut row = 0i32;

        while row < h {
            while col < w {
                if flip {
                    self.canvas
                        .borrow_mut()
                        .set_draw_color(Color::RGB(100, 100, 100));
                } else {
                    self.canvas
                        .borrow_mut()
                        .set_draw_color(Color::RGB(80, 80, 80));
                }
                self.fill_rectangle(col, row, col + size, row + size);
                flip = !flip;

                col += size;
            }
            flip = !flip;
            col = 0;
            row += size;
        }
    }

    // ----------------------------------------------------------
    // State management
    // ----------------------------------------------------------
    pub fn print_stack(&self, to_depth: usize) {
        print_stack(&self.state, self.stack_top, to_depth);
    }

    // Push the current transform onto the stack
    pub fn save(&mut self) {
        {
            let top = &mut self.state[self.stack_top];

            top.clear_color = self.clear_color;
            top.draw_color = self.draw_color;
            top.current = self.current_aft;
        }
        // println!("Context save:");
        // print_stack(&self.state, self.stack_top, 10);

        self.stack_top += 1;
    }

    pub fn apply(&mut self, aft: &AffineTransform) {
        // Concat this transform onto the current transform but don't push it.
        // Post multiply
        AffineTransform::multiply_mn(&aft, &self.current_aft, &mut self.post);
        self.current_aft = self.post;
    }

    /// Restores rendering state
    pub fn restore(&mut self) {
        self.stack_top -= 1;

        // println!("Context restore:");
        // print_stack(&self.state, self.stack_top, 10);

        let top = self.state[self.stack_top];

        self.clear_color = top.clear_color;
        self.draw_color = top.draw_color;
        self.current_aft = top.current;

        let mut can = self.canvas.borrow_mut();
        can.set_draw_color(self.draw_color);
    }

    // ----------------------------------------------------------
    // Transforms
    // ----------------------------------------------------------
    pub fn transform(&self, vertices: &Vec<RPoint>, bucket: &RefCell<Vec<RPoint>>) {
        // let mut transformed = self.bucket.borrow_mut();

        // for (i, vertex) in vertices.iter().enumerate() {
        //     AffineTransform::transform_to_point(&vertex, &mut transformed[i], &self.current_aft);
        // }
        let mut b = bucket.borrow_mut();
        for (i, vertex) in vertices.iter().enumerate() {
            AffineTransform::transform_to_point(&vertex, &mut b[i], &self.current_aft);
        }
    }

    pub fn top_index(&self) -> usize {
        self.stack_top
    }

    pub fn current(&mut self) -> &mut AffineTransform {
        &mut self.current_aft
    }

    pub fn previous(&mut self) -> &mut AffineTransform {
        &mut self.state[self.stack_top - 1].current
    }

    // -------------------------------------------------------------
    // Render primitives
    // All render methods are affected by the current matrix context.
    // -------------------------------------------------------------
    pub fn post(&self) {
        self.canvas.borrow_mut().present();
    }

    pub fn render_points(&self, vertices: &RefCell<Vec<RPoint>>) {
        let v = vertices.borrow();
        let mut can = self.canvas.borrow_mut();

        for p in v.iter() {
            match can.draw_point(Point::new(p.x as i32, p.y as i32)) {
                Err(err) => {
                    dbg!(err);
                }
                _ => (),
            }
        }
    }

    pub fn render_line(&self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let mut can = self.canvas.borrow_mut();
        match can.draw_line(
            Point::new(x1 as i32, y1 as i32),
            Point::new(x2 as i32, y2 as i32),
        ) {
            Err(err) => {
                dbg!(err);
            }
            _ => (),
        }
    }

    pub fn render_lines(&self, vertices: &RefCell<Vec<RPoint>>) {
        let v = vertices.borrow();
        let mut can = self.canvas.borrow_mut();
        let mut capture = true;
        let mut v1 = Point::new(0, 0);

        for p in v.iter() {
            if capture {
                v1 = Point::new(p.x as i32, p.y as i32);
                capture = false;
            } else {
                match can.draw_line(v1, Point::new(p.x as i32, p.y as i32)) {
                    Err(err) => {
                        dbg!(err);
                    }
                    _ => (),
                }
                capture = true;
            }
        }
    }

    // pub fn render_polygon(&self, vertices: &Vec<RPoint>) {
    //     let mut raster = self.tri_rasterizer.borrow_mut();

    // }

    pub fn render_triangle(&self, vertices: &RefCell<Vec<RPoint>>) {
        // Update visual with transformed vertices.
        let v = vertices.borrow();
        // let mut raster = self.tri_rasterizer.borrow_mut();
        // raster.set(v[0].x, v[0].y, v[1].x, v[1].y, v[2].x, v[2].y);
        // raster.draw(&self);

        let mut raster = self.fx_rasterizer.borrow_mut();
        raster.set(v[0].x, v[0].y, v[1].x, v[1].y, v[2].x, v[2].y);
        raster.draw(&self);
    }

    pub fn render_rectangle(&self, vertices: &RefCell<Vec<RPoint>>) {
        // Update visual with transformed vertices.
        let v = vertices.borrow();
        // let mut raster = self.tri_rasterizer.borrow_mut();
        let mut raster = self.fx_rasterizer.borrow_mut();

        // 1st triangle
        // raster.set(v[0].x, v[0].y, v[1].x, v[1].y, v[2].x, v[2].y);  // unshared edges
        // CCW
        raster.set(v[0].x, v[0].y, v[2].x, v[2].y, v[3].x, v[3].y);
        // CW
        // raster.set(v[0].x, v[0].y, v[3].x, v[3].y, v[2].x, v[2].y);
        raster.draw(&self);

        // 2nd triangle
        // raster.set(v[3].x, v[3].y, v[4].x, v[4].y, v[5].x, v[5].y);  // unshared edges
        // CCW
        raster.set(v[0].x, v[0].y, v[1].x, v[1].y, v[2].x, v[2].y);
        // CW
        // raster.set(v[0].x, v[0].y, v[2].x, v[2].y, v[1].x, v[1].y);
        raster.draw(&self);
    }

    // The vertices are expected to be the corners of an axis aligned bounding box.
    pub fn render_aabb_rectangle(&self, corners: &AABBox, filled: RenderStyle) {
        // Update visual with transformed vertices.
        // upper-left
        let minx = corners.min.x as i32;
        let miny = corners.min.y as i32;

        // bottom-right
        let maxx = corners.max.x as i32;
        let maxy = corners.max.y as i32;

        match filled {
            RenderStyle::FILLED => self.fill_rectangle(minx, miny, maxx, maxy),
            RenderStyle::OUTLINE => self.draw_rectangle(minx, miny, maxx, maxy),
            RenderStyle::BOTH => {
                self.fill_rectangle(minx, miny, maxx, maxy);
                self.draw_rectangle(minx, miny, maxx, maxy);
            }
        }
    }

    // ############################################################################
    // ############################################################################
    // ############################################################################
    // -------------------------------------------------------------
    // Draw primitives
    // All draw methods are NOT affected by the current matrix context.
    // Everything drawn here is written directly to device-space.
    // -------------------------------------------------------------
    /// x,y should already be transformed.
    #[inline(always)]
    pub fn set_pixel(&self, x: i32, y: i32) {
        let mut can = self.canvas.borrow_mut();
        match can.draw_point(Point::new(x, y)) {
            Err(err) => {
                dbg!(err);
            }
            _ => (),
        }
    }

    pub fn draw_rectangle(&self, xmin: i32, ymin: i32, xmax: i32, ymax: i32) {
        let mut can = self.canvas.borrow_mut();
        match can.draw_rect(Rect::new(
            xmin,
            ymin,
            (xmax - xmin) as u32,
            (ymax - ymin) as u32,
        )) {
            Err(err) => {
                dbg!(err);
            }
            _ => (),
        }
    }

    pub fn fill_rectangle(&self, xmin: i32, ymin: i32, xmax: i32, ymax: i32) {
        let mut can = self.canvas.borrow_mut();
        match can.fill_rect(Rect::new(
            xmin,
            ymin,
            (xmax - xmin) as u32,
            (ymax - ymin) as u32,
        )) {
            Err(err) => {
                dbg!(err);
            }
            _ => (),
        }
    }

    pub fn draw_point(&mut self, _x: f64, _y: f64) {
        // transform coordinates and render.
    }

    #[inline]
    pub fn draw_horz_line(&self, x1: i32, x2: i32, y: i32) {
        match self
            .canvas
            .borrow_mut()
            .draw_line(Point::new(x1, y), Point::new(x2, y))
        {
            Err(err) => {
                dbg!(err);
            }
            _ => (),
        }
    }

    // x,y are in view-space coordinates
    pub fn draw_horz_line_color(&mut self, x1: i32, x2: i32, y: i32) {
        let mut can = self.canvas.borrow_mut();
        match can.draw_line(Point::new(x1, y), Point::new(x2, y)) {
            Err(err) => {
                dbg!(err);
            }
            _ => (),
        }
    }

    pub fn draw_vert_line(&mut self, y1: i32, y2: i32, x: i32) {
        let mut can = self.canvas.borrow_mut();
        match can.draw_line(Point::new(x, y1), Point::new(x, y2)) {
            Err(err) => {
                dbg!(err);
            }
            _ => (),
        }
    }

    pub fn text(&mut self, x: i32, y: i32, text: &str, scale: usize, fill: usize) {
        // if x as usize >= self.world_properties.window_width
        //     || y as usize >= self.world_properties.window_height
        // {
        //     return;
        // }

        let mut cx = x;

        for c in text.chars() {
            let mut gy = y; // move y back to the "top" for each char
            if let Some(glyph) = BASIC_FONTS.get(c) {
                for g in &glyph {
                    let mut gx = cx; // set to current column
                    for bit in 0..8 {
                        // scan each pixel in the row
                        match *g & 1 << bit {
                            0 => (),
                            _ => {
                                if scale == 1 {
                                    self.set_pixel(gx, gy);
                                } else {
                                    let mut fillet = fill;
                                    if fill > scale {
                                        fillet = 0;
                                    }
                                    for xl in 0..((scale - fillet) as i32) {
                                        for yl in 0..((scale - fillet) as i32) {
                                            self.set_pixel(gx + xl, gy + yl);
                                        }
                                    }
                                }
                            }
                        }
                        gx += scale as i32;
                    }
                    gy += scale as i32; // move to next pixel-row in char
                }
            }
            cx += 8 * scale as i32; // move to next column/char/glyph
        }
    }
}

fn print_stack(state: &Vec<State>, stack_top: usize, to_depth: usize) {
    println!("Stack --------------------------");
    for i in 0..to_depth {
        let item = state[i];
        if i == stack_top {
            println!("({:?}) <<#### ({})", item.current, i);
        } else {
            println!("({:?})", item.current);
        }
    }
    println!("================================");
}
