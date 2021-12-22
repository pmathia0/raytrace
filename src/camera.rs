extern crate math;
use math::vector::{Vec3, Normalize};

use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3<f32>,
    pub lower_left_corner: Vec3<f32>,
    pub horizontal: Vec3<f32>,
    pub vertical: Vec3<f32>
}

impl Camera {
    pub fn new(lookfrom: Vec3<f32>, lookat: Vec3<f32>, vup: Vec3<f32>, vfov: f32, aspect_ratio: f32) -> Self {
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = (lookfrom - lookat).normalize();
        let u = (vup.cross(w)).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal/2f32 - vertical/2f32 - w,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin)
    }
}