pub mod ray;
pub mod sphere;
pub mod camera;
pub mod material;
pub mod hit;

use math::vector::{Vec3, Length, Normalize};
use rand::distributions::{Distribution, Uniform};

const MAX_RAND: u32 = 100_000;

pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..MAX_RAND);
    die.sample(&mut rng) as f32 / MAX_RAND as f32
}

pub fn random_f32_with_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let die = Uniform::from((min*MAX_RAND as f32).floor()..(max*MAX_RAND as f32).floor());
    die.sample(&mut rng) as f32 / MAX_RAND as f32
}

pub fn vec3_random() -> Vec3<f32> {
    Vec3::<f32>::new(random_f32(),random_f32(),random_f32())
}

pub fn vec3_random_with_range(min: f32, max: f32) -> Vec3<f32> {
    Vec3::<f32>::new(random_f32_with_range(min, max),random_f32_with_range(min, max),random_f32_with_range(min, max))
}

pub fn vec3_random_in_unit_sphere() -> Vec3<f32> {
    loop {
        let p = vec3_random_with_range(-1f32,1f32);
        if p.length() * p.length() >= 1.0 { continue; }
        return p;
    }
}

pub fn vec3_random_in_unit_disk() -> Vec3<f32> {
    loop {
        let p = Vec3::<f32>::new(random_f32_with_range(-1f32,1f32), random_f32_with_range(-1f32,1f32), 0.0);
        if p.length() * p.length() >= 1.0 { continue; }
        return p;
    }
}

pub fn vec3_random_unit() -> Vec3<f32> {
    let v = vec3_random_in_unit_sphere().normalize();
    v
}

pub fn vec3_near_zero(v: &Vec3<f32>) -> bool {
    let s: f32 = 1e-8;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

pub fn reflect(v: Vec3<f32>, n: Vec3<f32>) -> Vec3<f32> {
    let d = v.dot(n);
    v - n*d*2.0
}

pub fn refract(uv: Vec3<f32>, n: Vec3<f32>, etai_over_etat: f32) -> Vec3<f32> {
    let cos_theta = f32::min((uv*-1.0).dot(n),1.0);
    let r_out_perp = (uv + n*cos_theta) * etai_over_etat;
    let r_out_parallel = n*(-1.0*f32::sqrt(f32::abs(1.0 - r_out_perp.length()*r_out_perp.length() ) ));
    r_out_perp + r_out_parallel
}