use std::f64::consts::PI;
use std::ops::{Neg, Mul, Add, Sub, Div};


const EPSILON: f64 = 1e-5;


#[derive(Copy, Clone)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn vector(x: f64, y: f64, z: f64) -> Vector {
    Vector{x, y, z}
}

fn radians(degrees: f64) -> f64 {
    PI * degrees / 180.0
}

impl Vector {
    fn length(self) -> f64 {
        (self.x*self.x + self.y*self.y + self.y*self.y).sqrt()
    }

    fn norm(self) -> Self {
        let invert = 1.0 / self.length();
        self*invert
    }

    fn dot(self, rhs: Self) -> f64 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }

    fn clamp(self, min: f64, max: f64) -> Self {
        vector(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max)
        )
    }

    fn pow(self, power: f64) -> Self {
        vector(
            self.x.powf(power),
            self.y.powf(power),
            self.z.powf(power)
        )
    }

    fn splat(scalar: f64) -> Vector {
        vector(scalar, scalar, scalar)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        vector(-self.x, -self.y, -self.z)
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        vector(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        vector(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        vector(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        vector(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Div for Vector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        vector(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        vector(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}