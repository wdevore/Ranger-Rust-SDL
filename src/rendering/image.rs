use std::ops::{Index, IndexMut};

pub struct ImgPoint {
    // components are usize because there shouldn't be negative
    // indexs into an Image
    pub x: usize,
    pub y: usize,
}

impl ImgPoint {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn from_xy(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }

    pub fn from_point(p: &Self) -> Self {
        Self { x: p.x, y: p.y }
    }

    pub fn set_xy(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn set_point(&mut self, p: &Self) {
        self.x = p.x;
        self.y = p.y;
    }
}

// ######################################################
// Rectangle
// ######################################################
pub struct ImgRectangle {
    pub min: ImgPoint,
    pub max: ImgPoint,
}

impl ImgRectangle {
    pub fn new() -> Self {
        Self {
            min: ImgPoint { x: 0, y: 0 },
            max: ImgPoint { x: 1, y: 1 },
        }
    }

    pub fn from_size(w: usize, h: usize) -> Self {
        Self {
            min: ImgPoint { x: 0, y: 0 },
            max: ImgPoint { x: w, y: h },
        }
    }

    pub fn size(&self) -> usize {
        self.w() + self.h()
    }

    pub fn w(&self) -> usize {
        (self.max.x - self.min.x) as usize
    }

    pub fn h(&self) -> usize {
        (self.max.y - self.min.y) as usize
    }

    pub fn set_min_max(&mut self, min: &ImgPoint, max: &ImgPoint) {
        self.min.set_point(min);
        self.max.set_point(max);
    }

    pub fn point_inside(&self, p: &ImgPoint) -> bool {
        self.xy_inside(p.x, p.y)
    }

    pub fn xy_inside(&self, x: usize, y: usize) -> bool {
        self.min.x <= x && x < self.max.x && self.min.y <= y && y < self.max.y
    }
}

// R,G,B,A = 4 bytes
const BYTES_PER_PIXEL: usize = 4;

// RGBA is an in-memory image whose `at` method returns RGBA values.
pub struct RGBA {
    /// Pix holds the image's pixels, in R, G, B, A order.
    /// The pixel at (x, y) starts at
    /// Pix[(y-Rect.Min.Y)*Stride + (x-Rect.Min.X)*4].
    pub pix: Vec<u8>,
    /// Stride is the Pix stride (in bytes) between vertically adjacent pixels.
    pub stride: usize,
    /// Rect is the image's bounds.
    pub rect: ImgRectangle,
}

impl RGBA {
    pub fn new(w: usize, h: usize) -> Self {
        let mut buf: Vec<u8> = vec![];
        buf.clear();
        buf.resize(BYTES_PER_PIXEL * w * h, 0);

        Self {
            pix: buf,
            stride: BYTES_PER_PIXEL * w,
            rect: ImgRectangle::from_size(w, h),
        }
    }

    pub fn with_rectangle(r: ImgRectangle) -> Self {
        RGBA::new(r.w(), r.h())
    }

    pub fn buf(&mut self) -> &mut Vec<u8> {
        &mut self.pix
    }

    pub fn size(&mut self) -> usize {
        self.buf().len()
    }

    pub fn bounds(&self) -> &ImgRectangle {
        &self.rect
    }

    /// Returns the index of the first element of `pix`
    /// that corresponds to the pixel at (x, y).
    #[inline(always)]
    pub fn pix_offset(&self, x: usize, y: usize) -> usize {
        (y - self.rect.min.y) * self.stride + (x - self.rect.min.x) * BYTES_PER_PIXEL
    }

    pub fn at(&self, x: usize, y: usize) -> [u8; 4] {
        if !self.rect.xy_inside(x, y) {
            [0; 4]
        } else {
            let i = self.pix_offset(x, y) as usize;
            [
                self.pix[i + 0],
                self.pix[i + 1],
                self.pix[i + 2],
                self.pix[i + 3],
            ]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: [u8; 4]) {
        if self.rect.xy_inside(x, y) {
            let i = self.pix_offset(x, y) as usize;
            self.pix[i] = color[0];
            self.pix[i + 1] = color[1];
            self.pix[i + 2] = color[2];
            self.pix[i + 3] = color[3];
        }
    }

    pub fn set_components(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8, a: u8) {
        if self.rect.xy_inside(x, y) {
            let i = self.pix_offset(x, y) as usize;
            self.pix[i] = r;
            self.pix[i + 1] = g;
            self.pix[i + 2] = b;
            self.pix[i + 3] = a;
        }
    }
}

// #######################################################
// Raw color component access
// #######################################################
impl Index<usize> for RGBA {
    type Output = u8;

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.pix[index]
    }
}

impl IndexMut<usize> for RGBA {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut u8 {
        &mut self.pix[index]
    }
}
