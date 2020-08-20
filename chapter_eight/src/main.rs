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

extern crate rayon;
extern crate rand;
use rand::prelude::*;
use rayon::prelude::*;

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
    let nx = 3840;
    let ny = 2160;
    let ns = 40;
    let mut file = File::create("ch8.ppm")?;

    let first_line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write(first_line.as_bytes());

    let world = random_scene();

    let lookfrom = Vec3::new(8.0,2.0,3.0);
    let lookat = Vec3::new(4.0, 1.0, 1.5);
    let dist_to_focus : f32 = (lookfrom - lookat).length();
    let aperture : f32 = 0.12;
    // book cam
    let camera = Camera::new ( lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 30.0, nx as f32 / ny as f32, aperture, dist_to_focus);


    //dbg!(camera);
    (0..ny).into_par_iter().rev().flat_map(|y|
        (0..nx).flat_map(|x| {
            let col: Vec3 = (0..ns).map(|_| {
                let u = (i as f32 + rand::thread_rng().gen_range(0.0,1.0)) / nx as f32;
                let v = (j as f32 + rand::thread_rng().gen_range(0.0,1.0)) / ny as f32;

                let r = camera.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                color(r, &world, 0);
            }).sum();
            col.iter().map(|c|
                (255.99 * ( c / ns as f32).sqrt().max(0.0).min(1.0)) as u8
            ).collect::<Vec3>()
        }).collect::<Vec3>()
    ).collect::<Vec3>();
        

    for j in (0..ny).rev() {
        println!("Scanlines remaining : {}", j);
        //file.write(format!("{}\n", j).as_bytes());
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u = (i as f32 + rand::thread_rng().gen_range(0.0,1.0)) / nx as f32;
                let v = (j as f32 + rand::thread_rng().gen_range(0.0,1.0)) / ny as f32;

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
