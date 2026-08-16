use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Point {
        return Point { x, y };
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    #[inline]
    pub fn from_angle(angle: f32) -> Point {
        let (sin, cos) = f32::sin_cos(angle);
        Self { x: cos, y: sin }
    }

    #[inline]
    pub fn length(self) -> f32 {
        f32::sqrt(self.dot(self))
    }

    #[inline]
    pub fn length_recip(self) -> f32 {
        self.length().recip()
    }

    #[inline]
    pub fn normalize(self) -> Point {
        self.mul(self.length_recip())
    }
}

impl Add for Point {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Point;
    #[inline]
    fn mul(self, rhs: f32) -> Point {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl Mul<Point> for f32 {
    type Output = Point;
    #[inline]
    fn mul(self, rhs: Point) -> Point {
        Point {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}
