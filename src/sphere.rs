extern crate math;
use math::vector::Vec3;

use crate::ray::{Hitable, HitRecord};

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
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32, hit_record: &mut crate::ray::HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction()) * 2.0;
        let c = oc.dot(oc) - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.point_at_parameter(hit_record.t);
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                return true;
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.point_at_parameter(hit_record.t);
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                return true
            }
        }
        false
    }
}

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> Self {
        HitableList {
            list
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32, hit_record: &mut crate::ray::HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in 0..self.list.len() {
            if self.list[i].hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec;
            }
        }
        hit_anything
    }
}