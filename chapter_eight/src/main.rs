mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::*;
use hitable::*;
use hitable_list::*;

use material::*;
use ray::*;
use sphere::*;
use std::fs::File;
use std::io::prelude::*;
use vec3::*;

extern crate rand;
use rand::prelude::*;

// trait bound here, putting the Trait right into the argument isn't allowed
fn color(r: Ray, world: &HitableList, depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some(s) = hit.material.scatter(r, hit.clone()) {
                return s.attenuation * color(s.scattered, world, depth + 1);
            }
        }
        return Vec3::new (0.0, 0.0, 0.0);
    }
    else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = (unit_direction.y() + 1.0) * 0.5;
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
    
}

fn main() -> std::io::Result<()> {
    let nx = 500;
    let ny = 250;
    let ns = 100;
    let mut file = File::create("ch8.ppm")?;

    let first_line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write(first_line.as_bytes());

    let mut list: Vec<Box<dyn Hitable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian{albedo: Vec3::new(0.8, 0.3, 0.3)}))));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian{albedo: Vec3::new(0.8, 0.8, 0.0)}))));
    list.push(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal{albedo: Vec3::new(0.2, 0.4, 0.8), fuzz: 0.0}))));
    list.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Dielectric{ref_idx : 1.5}))));
    //list.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Box::new(Dielectric{ref_idx : 1.5}))));

    let world = HitableList::new(list, 2);

    let camera = Camera {
        lower_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::new(0.0, 0.0, 0.0),
    };

    for j in (0..ny).rev() {
        println!("Scanlines remaining : {}", j);
        //file.write(format!("{}\n", j).as_bytes());
        for i in 0..nx {
            let mut rng = rand::thread_rng();
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let mut random: f32 = rng.gen();
                //let r2: f32 = rng.gen();
                let u = (i as f32 + random) / nx as f32;
                random = rng.gen();
                let v = (j as f32 + random) / ny as f32;

                let r = camera.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                col += color(r, &world, 0);
            }
            col /= ns as f32;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

            let ir = 255.99 * col[0];
            let ig = 255.99 * col[1];
            let ib = 255.99 * col[2];
            file.write(format!("{} {} {}\n", ir as i32, ig as i32, ib as i32).as_bytes());
        }
    }

    Ok(())
}
