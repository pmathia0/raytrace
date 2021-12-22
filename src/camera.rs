extern crate math;
use math::vector::{Vec3, Normalize};

use crate::{ray::Ray, vec3_random_in_unit_disk};

pub struct Camera {
    pub origin: Vec3<f32>,
    pub lower_left_corner: Vec3<f32>,
    pub horizontal: Vec3<f32>,
    pub vertical: Vec3<f32>,
    pub u: Vec3<f32>,
    pub v: Vec3<f32>,
    pub w: Vec3<f32>,
    pub lens_radius: f32
}

impl Camera {
    pub fn new(
        lookfrom: Vec3<f32>,
        lookat: Vec3<f32>,
        vup: Vec3<f32>,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32) -> Self {

        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = (lookfrom - lookat).normalize();
        let u = (vup.cross(w)).normalize();
        let v = w.cross(u);

        let origin = lookfrom;

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal/2f32 - vertical/2f32 - w*focus_dist,
            u,v,w,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = vec3_random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset)
    }
}