mod camera;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;
extern crate rand;

use camera::*;
use hitable::*;
use hitable_list::*;
use rand::prelude::*;
use ray::*;
use sphere::*;
use std::fs::File;
use std::io::prelude::*;
use vec3::*;

fn random_in_unit_sphere() -> Vec3{
    let mut rng = rand::thread_rng();
    loop {
        let mut r1: f32 = rng.gen();
        let mut r2: f32 = rng.gen();
        let mut r3: f32 = rng.gen();
        let p = Vec3::new(r1, r2, r3) * 2.0 - Vec3::new(1.0, 1.0, 1.0) ;
        if p.squared_length() < 1.0 {
            return p
        }
    }
}

// trait bound here, putting the Trait right into the argument isn't allowed
fn color(r: Ray, world: &HitableList) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        
        let target = hit.p + hit.normal + random_in_unit_sphere();
        //println!("target: {}", target);
        return color( Ray::new(hit.p, target - hit.p), world) * 0.5;

    } else {
        let unit_direction = unit_vector(r.direction());
        let t = (unit_direction.y() + 1.0) * 0.5;
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

fn main() -> std::io::Result<()> {
    let nx = 500;
    let ny = 250;
    let ns = 100;
    let mut file = File::create("ch6.ppm")?;

    let first_line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write(first_line.as_bytes());

    let mut list: Vec<Box<dyn Hitable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
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
            let mut col = Vec3::new(0.0,0.0,0.0);
            for s in 0..ns {
                let mut random: f32 = rng.gen();
                //let r2: f32 = rng.gen();
                let u = (i as f32 + random) / nx as f32;
                random = rng.gen();
                let v = (j as f32 + random) / ny as f32;

                let r = camera.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                col += color(r, &world);
            }
            
            col /= ns as f32;
            col = Vec3::new (col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

            let ir = 255.99 * col[0];
            let ig = 255.99 * col[1];
            let ib = 255.99 * col[2];
            file.write(format!("{} {} {}\n", ir as i32, ig as i32, ib as i32).as_bytes());
        }
    }

    Ok(())
}
