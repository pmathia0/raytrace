extern crate math;

use math::vector::Vec3;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3<f32>,
    pub normal: Vec3<f32>
}

impl Default for HitRecord {
    fn default() -> Self {
        Self { t: 0.0, p: Vec3::<f32>::zero(), normal: Vec3::<f32>::zero() }
    }
}

pub struct Ray {
    pub a: Vec3<f32>,
    pub b: Vec3<f32>
}

impl Ray {
    
    pub fn new(a: Vec3<f32>, b: Vec3<f32>) -> Self {
        Ray { a, b }
    }

    pub fn origin(&self) -> Vec3<f32> {
        self.a
    }

    pub fn direction(&self) -> Vec3<f32> {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3<f32> {
        self.a + self.b * t
    }
}