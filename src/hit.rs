use std::rc::Rc;

use math::vector::Vec3;

use crate::{ray::Ray, material::{Material, Lambertian}};

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord);
}

pub struct HitRecord {
    pub p: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub mat_ptr: Rc<Box<dyn Material>>,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3<f32>) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal*-1.0
        };
    }
}
impl Default for HitRecord {
    fn default() -> Self {
        Self {
            t: 0.0,
            p: Vec3::<f32>::zero(),
            normal: Vec3::<f32>::zero(),
            front_face: false,
            mat_ptr: Rc::new(Box::new(Lambertian::new(Vec3::<f32>::zero())))
        }
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
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = HitRecord::default();
        for i in 0..self.list.len() {
            let (is_hit, tmp_rec) = self.list[i].hit(r, t_min, closest_so_far);
            if is_hit {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                rec = tmp_rec;
            }
        }
        (hit_anything, rec)
    }
}