extern crate math;

use math::vector::Vec3;

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