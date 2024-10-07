use crate::vec3::Vec3;

use crate::vec3::Point3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin() + (t * self.direction())
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Point3::default(),
            dir: Vec3::default(),
        }
    }
}
