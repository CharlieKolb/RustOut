use std::ops::{ Add, Sub, Mul, Div };

#[derive(Debug, PartialEq, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(f64::powf(self.x, 2.0) + f64::powf(self.y, 2.0))
    }

    pub fn set_length(mut self, new_length: f64) -> Self {
        let len = self.length();
        self.x = self.x / len * new_length;
        self.y = self.y / len * new_length;
        self
    }
}

impl<'a, 'b> Add<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;

    fn add(self, other: &'b Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl<'a, 'b> Sub<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;

    fn sub(self, other: &'b Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl<'a> Mul<f64> for &'a Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f64) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl<'a> Div<f64> for &'a Vec2 {
    type Output = Vec2;

    fn div(self, scalar: f64) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}