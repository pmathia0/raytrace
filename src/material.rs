extern crate math;

use math::vector::{Vec3, Normalize};
use rand::random;

use crate::{ray::Ray, hit::HitRecord, vec3_random_unit, vec3_near_zero, reflect, vec3_random_in_unit_sphere, refract, random_f32};


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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3<f32>) -> (bool, Ray) {
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
    pub albedo: Vec3<f32>,
    pub fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vec3<f32>, f: f32) -> Self {
        Metal {
            albedo,
            fuzz: if f < 1.0 {f} else {1.0}
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3<f32>) -> (bool, Ray) {
        let reflected = reflect(r_in.direction().normalize(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + vec3_random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;
        (scattered.direction().dot(rec.normal) > 0.0, scattered)
    }
}

pub struct Dielectric {
    pub ir: f32 // index of refraction
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Dielectric {
            ir: index_of_refraction
        }
    }

    pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0*r0;
        r0 + (1.0-r0)*f32::powi(1.0-cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3<f32>) -> (bool, Ray) {
        *attenuation = Vec3::<f32>::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0/self.ir } else { self.ir };
        let unit_direction = r_in.direction().normalize();
        let cos_theta = f32::min((unit_direction*-1.0).dot(rec.normal),1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta*cos_theta);
        let cannot_refract = if refraction_ratio * sin_theta > 1.0 { true } else { false };

        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f32(){
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction);
        (true, scattered)
    }
}