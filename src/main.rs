extern crate math;
extern crate rand;

use math::vector::{ Vec3, Normalize };

use std::fs;
use rand::distributions::{Distribution, Uniform};

use raytrace::{ray::*, sphere::{Sphere, HitableList}, camera::Camera};
fn color(r: &Ray, world: &dyn Hitable) -> Vec3<f32> {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, f32::MAX, &mut rec) {
        return Vec3::<f32>::new(rec.normal.x+1.0,rec.normal.y+1.0,rec.normal.z+1.0)*0.5;
    } else {
        let unit_direction = r.direction().normalize();
        let t = (unit_direction.y + 1.0) * 0.5;
        return Vec3::<f32>::new(1.0,1.0,1.0)*(1.0-t) + Vec3::<f32>::new(0.5,0.7,1.0)*t;
    }
}

fn main() {
    println!("Hello, world!");

    let nx = 800u32;
    let ny = 400u32;
    let ns = 100u32;

    let mut data = String::new();
    data.push_str("P3\n");
    data.push_str(nx.to_string().as_str());
    data.push_str(" ");
    data.push_str(ny.to_string().as_str());
    data.push_str("\n");
    data.push_str(255.to_string().as_str());
    data.push_str("\n");

    let camera = Camera::new();

    let objects: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(Vec3::<f32>::new(0.0,0.0,-1.0), 0.5)),
        Box::new(Sphere::new(Vec3::<f32>::new(0.0,-100.5,-1.0), 100.0)),
    ];
    let list = HitableList::new(objects);

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..1000);
    
    for j in (0..(ny)).rev() {
        for i in 0..nx {
            let mut col = Vec3::<f32>::zero();
            for _s in 0..ns {
                let u = (i as f32 + die.sample(&mut rng) as f32 / 1000.0) / nx as f32;
                let v = (j as f32 + die.sample(&mut rng) as f32 / 1000.0) / ny as f32;
                let r = camera.get_ray(u, v);
                col = col + color(&r, &list);
            }
            col = col / ns as f32;
            let ir = (255.99*col.x) as u8;
            let ig = (255.99*col.y) as u8;
            let ib = (255.99*col.z) as u8;
            data.push_str(ir.to_string().as_str());
            data.push_str(" ");
            data.push_str(ig.to_string().as_str());
            data.push_str(" ");
            data.push_str(ib.to_string().as_str());
            data.push_str("\n");
        }
    }
    fs::write("output.ppm", data).unwrap();
}
