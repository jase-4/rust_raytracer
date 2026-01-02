use std::rc::Rc;

use crate::interval::Interval;
use crate::material::DefaultMaterial;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_normal_face(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = outward_normal.dot(&r.direction()) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            mat: Arc::new(DefaultMaterial::default()),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable : Sync + Send {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}
