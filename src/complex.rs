use std::ops::{Add, Sub, Mul, MulAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex {
    pub x: f64,
    pub y: f64,
}

impl Complex {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }    
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Self::Output {
        Self {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }
}

impl MulAssign<f64> for Complex {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
    }
}