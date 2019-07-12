use std::ops;

#[derive(Debug, PartialEq, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
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

impl ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f64) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, scalar: f64) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
        };
    }
}

impl ops::DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, scalar: f64) {
        *self = Self {
            x: self.x / scalar,
            y: self.y / scalar,
        };
    }
}
