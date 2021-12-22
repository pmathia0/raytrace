extern crate math;
use math::vector::Vec3;

use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3<f32>,
    pub lower_left_corner: Vec3<f32>,
    pub horizontal: Vec3<f32>,
    pub vertical: Vec3<f32>
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        let viewport_height = 2f32;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1f32;

        let origin = Vec3::<f32>::zero();
        let horizontal = Vec3::<f32>::new(viewport_width,0.0,0.0);
        let vertical = Vec3::<f32>::new(0.0,viewport_height,0.0);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal/2f32 - vertical/2f32 - Vec3::<f32>::new(0.0,0.0,focal_length),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin)
    }
}