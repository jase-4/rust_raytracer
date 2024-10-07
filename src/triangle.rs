use std::f64::EPSILON;

use crate::{
    hittable::Hittable,
    material::Material,
    vec3::{Point3, Vec3},
};
use std::rc::Rc;

pub struct Triangle {
    pub p0: Point3,
    pub p1: Point3,
    pub p2: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
}

impl Triangle {
    pub fn new(p0: Point3, p1: Point3, p2: Point3, mat: Rc<dyn Material>) -> Self {
        let edge1 = p1 - p0;
        let edge2 = p2 - p0;
        let normal = edge1.cross(&edge2).normalize();

        Self {
            p0,
            p1,
            p2,
            normal,
            mat,
        }
    }
}

impl Hittable for Triangle {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let epsilon = 1e-8;

        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;
        let h = r.direction().cross(&edge2);
        let a = edge1.dot(&h);

        if (a > -epsilon) && (a < epsilon) {
            return false;
        }

        let f = 1.0 / a;
        let s = r.origin() - self.p0;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return false;
        }

        let q = s.cross(&edge1);
        let v = f * r.direction().dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = f * edge2.dot(&q);

        if t > epsilon && t < ray_t.max {
            rec.t = t;
            rec.p = r.at(t);
            let outward_normal = self.normal;
            rec.set_normal_face(r, &outward_normal);
            rec.mat = self.mat.clone();

            return true;
        }

        false
    }
}

pub fn create_cube(material: Rc<dyn Material>, size: f64) -> Vec<Triangle> {
    let v0 = Point3::new(0.0, 0.0, 0.0);
    let v1 = Point3::new(size, 0.0, 0.0);
    let v2 = Point3::new(size, size, 0.0);
    let v3 = Point3::new(0.0, size, 0.0);
    let v4 = Point3::new(0.0, 0.0, size);
    let v5 = Point3::new(size, 0.0, size);
    let v6 = Point3::new(size, size, size);
    let v7 = Point3::new(0.0, size, size);

    vec![
        Triangle::new(v0, v1, v3, material.clone()),
        Triangle::new(v1, v2, v3, material.clone()),
        Triangle::new(v4, v5, v7, material.clone()),
        Triangle::new(v5, v6, v7, material.clone()),
        Triangle::new(v0, v3, v4, material.clone()),
        Triangle::new(v3, v7, v4, material.clone()),
        Triangle::new(v1, v2, v5, material.clone()),
        Triangle::new(v2, v6, v5, material.clone()),
        Triangle::new(v3, v2, v7, material.clone()),
        Triangle::new(v2, v6, v7, material.clone()),
        Triangle::new(v0, v1, v4, material.clone()),
        Triangle::new(v1, v5, v4, material.clone()),
    ]
}
