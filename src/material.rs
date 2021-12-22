extern crate math;

use math::vector::{Vec3, Normalize};

use crate::{ray::Ray, hit::HitRecord, vec3_random_unit, vec3_near_zero, reflect};


pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3<f32>) -> (bool, Ray);
}

pub struct Lambertian {
    pub albedo: Vec3<f32>
}

impl Lambertian {
    pub fn new(albedo: Vec3<f32>) -> Self {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3<f32>) -> (bool, Ray) {
        let mut scatter_direction = rec.normal + vec3_random_unit();

        // catch degenerate scatter direction
        if vec3_near_zero(&scatter_direction) {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        (true, scattered)
    }
}

pub struct Metal {
    pub albedo: Vec3<f32>
}

impl Metal {
    pub fn new(albedo: Vec3<f32>) -> Self {
        Metal {
            albedo
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3<f32>) -> (bool, Ray) {
        let reflected = reflect(&r_in.direction().normalize(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        (scattered.direction().dot(rec.normal) > 0.0, scattered)
    }
}