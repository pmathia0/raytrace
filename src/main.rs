extern crate math;
extern crate rand;
extern crate ktx2;

use ktx2::texture::TextureKtx2;
use ktx2::vk_format::VkFormat;
use math::vector::{ Vec3, Normalize };

use rand::distributions::{Distribution, Uniform};
use raytrace::{ray::*, sphere::Sphere, camera::Camera, vec3_random_unit, hit::{Hitable, HitRecord, HitableList}};

const NX: u32 = 600;
const NY: u32 = 400;
const NS: u32 = 100;
const MAX_DEPTH: i32 = 50;
const RAND_MAX: u32 = 100000;

fn ray_color(r: &Ray, world: &dyn Hitable, depth: i32) -> Vec3<f32> {
    let (is_hit, rec) = world.hit(r, 0.001, f32::MAX);
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3::<f32>::zero();
    }
    if is_hit {
        let target = rec.p + rec.normal + vec3_random_unit();
        return ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
    } else {
        let unit_direction = r.direction().normalize();
        let t = (unit_direction.y + 1.0) * 0.5;
        return Vec3::<f32>::new(1.0,1.0,1.0)*(1.0-t) + Vec3::<f32>::new(0.5,0.7,1.0)*t;
    }
}

#[inline(always)]
fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min
    }
    if x > max {
        return max
    }
    x
}

fn write_color(tex: &mut TextureKtx2, x: u32, y: u32, pixel_color: &Vec3<f32>, samples_per_pixel: u32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1f32 / samples_per_pixel as f32;
    r = (scale*r).sqrt();
    g = (scale*g).sqrt();
    b = (scale*b).sqrt();

    // write the translated [0,255] value of each color component
    let ir = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as u8;
    tex.write_pixel(x, NY-1-y, &[ir,ig,ib,255]);
}

fn main() {
    println!("Traycing the rays...");

    let camera = Camera::new(NX as f32 / NY as f32);

    let objects: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(Vec3::<f32>::new(0.0,0.0,-1.0), 0.5)),
        Box::new(Sphere::new(Vec3::<f32>::new(0.0,-100.5,-1.0), 100.0)),
    ];
    let world = HitableList::new(objects);

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..RAND_MAX);
    
    let mut tex: TextureKtx2 = TextureKtx2::new(NX, NY, VkFormat::R8G8B8A8_UNORM);
    for j in 0..NY {
        for i in 0..NX {
            println!("Processing row {}", j);
            let mut pixel_color = Vec3::<f32>::zero();
            for _s in 0..NS {
                let u = (i as f32 + die.sample(&mut rng) as f32 / RAND_MAX as f32) / NX as f32;
                let v = (j as f32 + die.sample(&mut rng) as f32 / RAND_MAX as f32) / NY as f32;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(&mut tex, i, j, &pixel_color, NS);
        }
    }
    tex.write_to_ktx2("output.ktx2").unwrap();
}
