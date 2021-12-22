extern crate math;

use math::vector::Vec3;

use crate::{ray::Ray, hit::HitRecord};


pub trait Material {
    fn scatter(self, r_in: &Ray, rec: &HitRecord, attenuation: &Vec3<f32>, scattered: &Ray) -> bool;
}