mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;

use vec3::*;
use ray::*;
use hitable::*;
use hitable_list::*;
use sphere::*;
use std::fs::File;
use std::io::prelude::*;

// trait bound here, putting the Trait right into the argument isn't allowed
fn color (r: Ray, world : &HitableList) -> Vec3 {

    if  let Some(hit) = world.hit(r, 0.0, std::f32::MAX) {
        return Vec3::new (hit.normal.x() + 1.0, hit.normal.y() + 1.0, hit.normal.z() + 1.0) * 0.5;
    }

    else {
        let unit_direction = unit_vector(r.direction());
        let t = (unit_direction.y() + 1.0) * 0.5;
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new (0.5, 0.7, 1.0) * t;
    }
}

fn unit_vector(v : Vec3) -> Vec3 {
    v / v.length()
}

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;
    let mut file = File::create ("ch6.ppm")?;

    let first_line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write(first_line.as_bytes());

    let lower_corner = Vec3::new (-2.0, -1.0, -1.0);
    let horizontal = Vec3::new (4.0, 0.0, 0.0);
    let vertical = Vec3::new (0.0, 2.0, 0.0);
    let origin = Vec3::new (0.0, 0.0, 0.0);

    let mut list : Vec<Box<dyn Hitable>> = Vec:: new();
    list.push(Box::new(Sphere::new (Vec3::new (0.0,0.0,-1.0), 0.5)));
    list.push(Box::new(Sphere::new (Vec3::new (0.0, -100.5, -1.0), 100.0)));
    let world = HitableList::new (list, 2);

    for j in (0..ny).rev() {
        //file.write(format!("{}\n", j).as_bytes());
        for i in 0..nx {
            let u = i as f32/ nx as f32;
            let v = j as f32/ ny as f32;

            let r = Ray::new (origin, lower_corner + horizontal * u + vertical * v);
            let col = color(r, &world);

            let ir = 255.99 * col[0];
            let ig = 255.99 * col[1];
            let ib = 255.99 * col[2];
            file.write(format!("{} {} {}\n", ir as i32, ig as i32, ib as i32).as_bytes());
        }
    }

    Ok(())
}
