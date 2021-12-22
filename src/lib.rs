pub mod ray;
pub mod sphere;
pub mod camera;

use math::vector::{Vec3, Length, Normalize};
use rand::distributions::{Distribution, Uniform};

const MAX_RAND: u32 = 100_000;

pub fn vec3_random() -> Vec3<f32> {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..MAX_RAND);
    Vec3::<f32>::new(die.sample(&mut rng) as f32 / MAX_RAND as f32,die.sample(&mut rng) as f32 / MAX_RAND as f32,die.sample(&mut rng) as f32 / MAX_RAND as f32)
}

pub fn vec3_random_with_range(min: f32, max: f32) -> Vec3<f32> {
    let mut rng = rand::thread_rng();
    let die = Uniform::from((min*MAX_RAND as f32).floor()..(max*MAX_RAND as f32).floor());
    Vec3::<f32>::new(die.sample(&mut rng) as f32 / MAX_RAND as f32,die.sample(&mut rng) as f32 / MAX_RAND as f32,die.sample(&mut rng) as f32 / MAX_RAND as f32)
}

pub fn vec3_random_in_unit_sphere() -> Vec3<f32> {
    loop {
        let p = vec3_random_with_range(-1f32,1f32);
        if p.length() * p.length() >= 1.0 { continue; }
        return p;
    }
}

pub fn vec3_random_unit() -> Vec3<f32> {
    let v = vec3_random_in_unit_sphere().normalize();
    v
}