extern crate math;
use std::rc::Rc;

use math::vector::{Vec3, Length};

use crate::{hit::{Hitable, HitRecord}, material::Material};

pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
    pub mat_ptr: Rc<Box<dyn Material>>
}

impl Sphere {
    pub fn new(center: Vec3<f32>, radius: f32, m: Rc<Box<dyn Material>>) -> Self {
        Sphere { center, radius, mat_ptr: m }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let oc = r.origin() - self.center;
        let a = r.direction().length()*r.direction().length();
        let half_b = oc.dot(r.direction());
        let c = oc.length()*oc.length() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 { return (false, HitRecord::default()) }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root { return (false, HitRecord::default()) }
        }
        let mut rec = HitRecord {
            t: root,
            p: r.point_at_parameter(root),
            normal: (r.point_at_parameter(root) - self.center) / self.radius,
            front_face: true,
            mat_ptr: Rc::clone(&self.mat_ptr)
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        return (true, rec);
    }
}

