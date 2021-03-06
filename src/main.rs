extern crate math;
extern crate rand;
extern crate ktx2;

use std::rc::Rc;

use ktx2::texture::TextureKtx2;
use ktx2::vk_format::VkFormat;
use math::vector::{ Vec3, Normalize, Length };

use rand::distributions::{Distribution, Uniform};
use raytrace::{ray::*, sphere::Sphere, camera::Camera, hit::{Hitable, HitableList}, material::{Lambertian, Material, Metal, Dielectric}, random_f32, vec3_random_with_range, random_f32_with_range};

const NX: u32 = 1200;
const NY: u32 =  800;
const NS: u32 = 50;
const MAX_DEPTH: i32 = 50;
const RAND_MAX: u32 = 100000;

fn ray_color(r: &Ray, world: &dyn Hitable, depth: i32) -> Vec3<f32> {
    let (is_hit, rec) = world.hit(r, 0.001, f32::MAX);
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3::<f32>::zero();
    }
    if is_hit {
        let mut attenuation = Vec3::<f32>::zero();
        let (is_sc, scattered) = rec.mat_ptr.scatter(r, &rec, &mut attenuation);
        if is_sc {
            return attenuation * ray_color(&scattered, world, depth-1);
        }
        return Vec3::<f32>::zero();
    }
    let unit_direction = r.direction().normalize();
    let t = (unit_direction.y + 1.0) * 0.5;
    return Vec3::<f32>::new(1.0,1.0,1.0)*(1.0-t) + Vec3::<f32>::new(0.5,0.7,1.0)*t;
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

    // Camera
    let lookfrom = Vec3::<f32>::new(13.0,2.0,3.0); 
    let lookat = Vec3::<f32>::new(0.0,0.0,0.0);
    let camera = Camera::new(
        lookfrom, 
        lookat,
        Vec3::<f32>::new(0.0,1.0,0.0), 
        20.0, 
        NX as f32 / NY as f32,
        0.1,
        10.0);

    // World
    let material_ground: Rc<Box<dyn Material>> = Rc::new(Box::new(Lambertian::new(Vec3::<f32>::new(0.5,0.5,0.5))));

    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();
    objects.push(Box::new(Sphere::new(Vec3::<f32>::new( 0.0,-1000.0,0.0), 1000.0, Rc::clone(&material_ground))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();
            let center = Vec3::<f32>::new(random_f32()*0.9 + a as f32, 0.2, random_f32()*0.9 + b as f32);

            if (center - Vec3::<f32>::new(4.0,0.2,0.0)).length() > 0.9 {
                let material: Box<dyn Material> =
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vec3_random_with_range(0.0, 1.0)*vec3_random_with_range(0.0, 1.0);
                    Box::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec3_random_with_range(0.5, 1.0);
                    let fuzz = random_f32_with_range(0.0, 0.5);
                    Box::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Box::new(Dielectric::new(1.5))
                };
                let sphere = Box::new(Sphere::new(center, 0.2, Rc::new(material)));
                objects.push(sphere);
            }
        }
    }
    let material1 = Box::new(Dielectric::new(1.5));
    objects.push(Box::new(Sphere::new(Vec3::<f32>::new(0.0,1.0,0.0), 1.0, Rc::new(material1))));
    let material2 = Box::new(Lambertian::new(Vec3::<f32>::new(0.4,0.2,0.1)));
    objects.push(Box::new(Sphere::new(Vec3::<f32>::new(-4.0,1.0,0.0), 1.0, Rc::new(material2))));
    let material3 = Box::new(Metal::new(Vec3::<f32>::new(0.7,0.6,0.5), 0.0));
    objects.push(Box::new(Sphere::new(Vec3::<f32>::new(4.0,1.0,0.0), 1.0, Rc::new(material3))));

    let world = HitableList::new(objects);

    // Render
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..RAND_MAX);
    
    let mut tex: TextureKtx2 = TextureKtx2::new(NX, NY, VkFormat::R8G8B8A8_UNORM);
    for j in 0..NY {
        println!("Processing row {}", j);
        for i in 0..NX {
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
