use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Neg, Sub};

const EPSILON: f64 = 1e-5;

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn radians(degrees: f64) -> f64 {
    PI * degrees / 180.0
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.y * self.y).sqrt()
    }

    pub fn norm(self) -> Self {
        let invert = 1.0 / self.length();
        self * invert
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn clamp(self, min: f64, max: f64) -> Self {
        Self::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        )
    }

    pub fn pow(self, power: f64) -> Self {
        Vector {
            x: self.x.powf(power),
            y: self.y.powf(power),
            z: self.z.powf(power),
        }
    }

    pub fn splat(scalar: f64) -> Vector {
        Vector::new(scalar, scalar, scalar)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vector::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Div for Vector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
