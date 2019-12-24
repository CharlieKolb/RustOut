use std::{ f64, ops };

static CMP_EPSILON : f64 = 0.000000000000001f64;

pub fn clamp<T: PartialOrd>(val: T, low: T, high: T) -> T {
    if val < low {
        return low;
    } else if val > high {
        return high;
    } else {
        return val;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Segment(pub Vec2, pub Vec2);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HitOrDistance {
    Hit(Vec2),
    Distance(f64),
}

// According to https://stackoverflow.com/a/1501725
pub fn point_segment_distance(point: &Vec2, s: &Segment) -> f64 {
    let segment = s.1 - s.0;
    let s2 = segment.length_squared();

    // case start == end
    let ls_to_p = point - &s.0;

    if s2 == 0. {
        return ls_to_p.length();
    }

    let t = clamp(ls_to_p.dot(&segment) / s2, 0., 1.);
    let projection = s.0 + t * segment;
    (*point - projection).length()
}

// Return distance and intersection point if distance is 0.
// According to https://stackoverflow.com/a/1968345
pub fn segment_segment_distance( 
    s1: &Segment,
    s2: &Segment,
) -> HitOrDistance {
    let a1 = s1.1 - s1.0;
    let a2 = s2.1 - s2.0;

    // potential div by 0!
    let s = (-a1.y * (s1.0.x - s2.0.x) + a1.x * (s1.0.y - s2.0.y)) / (-a2.x * a1.y + a1.x * a2.y);

    let t = (a2.x * (s1.0.y - s2.0.y) - a2.y * (s1.0.x - s2.0.x)) / (-a2.x * a1.y + a1.x * a2.y);

    if s >= 0. && s <= 1. && t >= 0. && t <= 1. {
        HitOrDistance::Hit(s1.0 + t * a1)
    } else {
        // no collision
        HitOrDistance::Distance(
            [
                point_segment_distance(&s1.0, &s2),
                point_segment_distance(&s1.1, &s2),
                point_segment_distance(&s2.0, &s1),
                point_segment_distance(&s2.1, &s1),
            ]
            .iter()
            .fold(f64::INFINITY, |a, &b| a.min(b))
        )
    }
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn to_norm(&self) -> Self {
        let l = self.length();
        if l == 0. {
            return Vec2::new(0., 0.);
        }
        self / self.length()
    }

    pub fn get_closer_normal(&self, to: &Vec2) -> Vec2 {
        let norm1 = Vec2::new(self.y, -self.x);
        let norm2 = Vec2::new(-self.y, self.x);
        let l1 = (norm1 - *to).length_squared();
        let l2 = (norm2 - *to).length_squared();
        if l1 <= l2 {
            norm1.to_norm()
        } else {
            norm2.to_norm()
        }
    }

    pub fn reflect_on(&self, normal: &Vec2) -> Vec2 {
        let normed = normal.to_norm();
        *self - 2. * normed * self.dot(&normed)
    }

    // Angle from 0; Pi
    pub fn get_angle_with(&self, other: &Vec2) -> f64 {
        f64::acos(self.to_norm().dot(&other.to_norm()))
    }

    pub fn to_rotated_rad(&self, rad: f64) -> Vec2 {
        let c = f64::cos(rad);
        let s = f64::sin(rad);
        let tx = self.x * c - self.y * s;
        let ty = self.y * s + self.y * c;
        Vec2::new(tx, ty)
    }

    pub fn to_rotated_deg(&self, deg: f64) -> Vec2 {
        self.to_rotated_rad(deg / 180. * f64::consts::PI) / f64::consts::PI * 180.
    }

    pub fn with_x(&self, x: f64) -> Self {
        Self { x, y: self.y }
    }

    pub fn with_y(&self, y: f64) -> Self {
        Self { x: self.x, y }
    }

    pub fn set_length(mut self, new_length: f64) -> Self {
        let len = self.length();
        self.x = self.x / len * new_length;
        self.y = self.y / len * new_length;
        self
    }

    pub fn dot(&self, o: &Vec2) -> f64 {
        self.x * o.x + self.y * o.y
    }

    pub fn cross(&self, o: &Vec2) -> f64 {
        self.x * o.y - self.y * o.x
    }

    pub fn equalish(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < CMP_EPSILON && (self.y - other.y).abs() < CMP_EPSILON
    }
}

impl ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl ops::Neg for &Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::Add for &Vec2 {
    type Output = Vec2;

    fn add(self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::Sub for &Vec2 {
    type Output = Vec2;

    fn sub(self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f64) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl ops::Mul<f64> for &Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f64) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl ops::Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Vec2 {
        v * self
    }
}

impl ops::Mul<&Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, v: &Vec2) -> Vec2 {
        v * self
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, scalar: f64) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}

impl ops::Div<f64> for &Vec2 {
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

impl ops::AddAssign<&Vec2> for Vec2 {
    fn add_assign(&mut self, other: &Vec2) {
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

impl ops::SubAssign<&Vec2> for Vec2 {
    fn sub_assign(&mut self, other: &Vec2) {
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

impl ops::Add<Vec2> for Segment {
    type Output = Segment;

    fn add(self, vec: Vec2) -> Segment {
        Segment(self.0 + vec, self.1 + vec)
    }
}

impl ops::Add<Vec2> for &Segment {
    type Output = Segment;

    fn add(self, vec: Vec2) -> Segment {
        Segment(self.0 + vec, self.1 + vec)
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_point_line_distance_00() {
        let point = Vec2::new(2.5, 0.);

        let line_start = Vec2::new(0., 2.);
        let line_end = Vec2::new(0., -4.);

        let result = point_segment_distance(&point, &Segment(line_start, line_end));

        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_point_line_distance_01() {
        let point = Vec2::new(5.5, 0.);
        let line = Segment(Vec2::new(5., 2.), Vec2::new(5., -4.));

        let result = point_segment_distance(&point, &line);

        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_segment_segment_distance_contact() {
        let s1 = Segment(Vec2::new(5., 0.), Vec2::new(6., 0.));
        let s2 = Segment(Vec2::new(5., 2.), Vec2::new(5., -4.));

        let result = segment_segment_distance(&s1, &s2);

        assert_eq!(result, HitOrDistance::Hit(Vec2::new(5., 0.)));
    }

    #[test]
    fn test_segment_segment_distance_no_contact() {
        let s1 = Segment(Vec2::new(-1., 0.), Vec2::new(6., 0.));
        let s2 = Segment(Vec2::new(-5., 2.), Vec2::new(-5., -4.));

        let result = segment_segment_distance(&s1, &s2);

        assert_eq!(result, HitOrDistance::Distance(4.0));
    }

    #[test]
    fn test_point_line_distance_02() {
        let s = Segment(Vec2::new(-1., 0.), Vec2::new(6., 0.));

        let point = Vec2::new(-5., 0.);

        let result = point_segment_distance(&point, &s);

        assert_eq!(result, 4.);
    }

    #[test]
    fn test_mirror_on_01() {
        let mirror = Vec2::new(1., 1.);
        let input = Vec2::new(2., 0.);
        let expected = Vec2::new(0., -2.);

        assert!(input.reflect_on(&mirror).equalish(&expected));
    }
}
