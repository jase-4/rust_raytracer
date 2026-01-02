use std::rc::Rc;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = oc.dot(&r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);
        let descriminant = h * h - a * c;
        if descriminant < 0.0 {
            return false;
        }

        let sqrtd = descriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surronds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surronds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_normal_face(r, &outward_normal);
        rec.mat = self.mat.clone();

        true
    }
}
