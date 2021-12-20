extern crate math;
use math::vector::Vec3;

use std::fs;

fn main() {
    println!("Hello, world!");

    let nx = 200u32;
    let ny = 100u32;

    let mut data = String::new();
    data.push_str("P3\n");
    data.push_str(nx.to_string().as_str());
    data.push_str(" ");
    data.push_str(ny.to_string().as_str());
    data.push_str("\n");
    data.push_str(255.to_string().as_str());
    data.push_str("\n");

    for j in (0..(ny)).rev() {
        for i in 0..nx {
            let col = Vec3::<f32>::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2f32);
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
