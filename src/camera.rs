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
    pub fn new() -> Self {
        Camera {
            origin: Vec3::<f32>::zero(),
            lower_left_corner: Vec3::<f32>::new(-2.0,-1.0,-1.0),
            horizontal: Vec3::<f32>::new(4.0,0.0,0.0),
            vertical: Vec3::<f32>::new(0.0,2.0,0.0)
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin)
    }
}