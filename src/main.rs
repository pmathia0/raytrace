extern crate math;
use math::vector::{ Vec3, Normalize };

use std::fs;
use raytrace::ray::Ray;

fn color(r: &Ray) -> Vec3<f32> {
    let t = hit_sphere(Vec3::<f32>::new(0.0,0.0,-1.0), 0.5, r);
    if t > 0f32 {
        let n = (r.point_at_parameter(t) - Vec3::<f32>::new(0.0,0.0,-1.0)).normalize();
        return Vec3::<f32>::new(n.x+1.0, n.y+1.0, n.z+1.0)*0.5;
    }
    let unit_direction: Vec3<f32> = r.direction().normalize();
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    Vec3::<f32>::new(1.0,1.0,1.0) * (1.0-t) + Vec3::<f32>::new(0.5,0.7,1.0) * t
}

fn hit_sphere(center: Vec3<f32>, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = oc.dot(r.direction()) * 2.0;
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        -1f32
    } else {
        (-b - discriminant.sqrt()) / (2f32*a)
    }
}

fn main() {
    println!("Hello, world!");

    let nx = 600u32;
    let ny = 300u32;

    let mut data = String::new();
    data.push_str("P3\n");
    data.push_str(nx.to_string().as_str());
    data.push_str(" ");
    data.push_str(ny.to_string().as_str());
    data.push_str("\n");
    data.push_str(255.to_string().as_str());
    data.push_str("\n");

    let lower_left_corner = Vec3::<f32>::new(-2.0,-1.0,-1.0);
    let horizontal = Vec3::<f32>::new(4.0,0.0,0.0);
    let vertical = Vec3::<f32>::new(0.0,2.0,0.0);
    let origin = Vec3::<f32>::zero();
    for j in (0..(ny)).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v);
            let col = color(&r);
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
