//use std::num;
use rand::{thread_rng, Rng};
use std::fmt;

//#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Point {
    x: u32,
    y: u32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[allow(dead_code)]
impl Point {
    pub fn from(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    pub fn from_random(x_max: u32, y_max: u32) -> Point {
        let mut rng = thread_rng();
        let x: u32 = rng.gen_range(0..x_max);
        let y: u32 = rng.gen_range(0..y_max);
        Point { x, y }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).pow(2) as f64 + (self.y - other.y).pow(2) as f64).sqrt()
    }
}
