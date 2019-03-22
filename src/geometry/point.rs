use std::cell::RefCell;
use std::ops::{Add, Sub};

pub type RVertices = RefCell<Vec<Point>>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new() -> Self {
        Self {
            x: f64::default(),
            y: f64::default(),
        }
    }

    pub fn from_xy(x: f64, y: f64) -> Self {
        Self { x: x, y: y }
    }

    pub fn from_tup(p: (f64, f64)) -> Self {
        Self { x: p.0, y: p.1 }
    }

    pub fn from_point(p: &Self) -> Self {
        Self { x: p.x, y: p.y }
    }

    pub fn set_xy(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn set_point(&mut self, p: &Self) {
        self.x = p.x;
        self.y = p.y;
    }

    pub fn translate(&mut self, p: &Self) {
        self.x += p.x;
        self.y += p.y;
    }

    pub fn distance_between(&self, other: &Self) -> f64 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// impl Mul for Point {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self::Output {
//         Self {
//             x: self.x * rhs.x,
//             y: self.y * rhs.y,
//         }
//     }
// }
