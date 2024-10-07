use crate::util::{self};
use rand::Rng;
use std::default::Default;

use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }

    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }
    pub fn unit_vector(&self) -> Vec3 {
        *self / (self.length())
    }

    pub fn length(&self) -> f64 {
        self.dot(&self).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        Vec3 {
            e: [self.x() / len, self.y() / len, self.z() / len],
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn near_zero(&self) -> bool {
        const EPSILON: f64 = 1e-8;
        (self.x().abs() < EPSILON) && (self.y().abs() < EPSILON) && (self.z().abs() < EPSILON)
    }

    pub fn random(min: f64, max: f64) -> Self {
        Vec3::new(
            util::random_double_range(min, max),
            util::random_double_range(min, max),
            util::random_double_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

pub type Point3 = Vec3;

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            e: [vec.x() * self, vec.y() * self, vec.z() * self],
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            e: [self.x() * scalar, self.y() * scalar, self.z() * scalar],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [self.x() * v.x(), self.y() * v.y(), self.z() * v.z()],
        }
    }
}

impl Mul<&f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: &f64) -> Vec3 {
        Vec3 {
            e: [self.x() * scalar, self.y() * scalar, self.z() * scalar],
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()],
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        if scalar == 0.0 {
            panic!("Division by zero");
        }
        Vec3 {
            e: [self.x() / scalar, self.y() / scalar, self.z() / scalar],
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p.normalize();
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / (v.length())
}
