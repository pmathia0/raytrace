extern crate math;
use math::vector::Vec3;

use crate::hit::{Hitable, HitRecord};

pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32
}

impl Sphere {
    pub fn new(center: Vec3<f32>, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        let mut hit_record = HitRecord::default();
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.point_at_parameter(hit_record.t);
                let outward_normal = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(r, outward_normal);
                return (true, hit_record);
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                let outward_normal = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(r, outward_normal);
                return (true, hit_record);
            }
        }
        return (false, hit_record);
    }
}

