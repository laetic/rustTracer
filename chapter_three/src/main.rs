mod vec3;
mod ray;

use vec3::*;
use ray::*;
use std::fs::File;
use std::io::prelude::*;

fn color(r: Ray) -> Vec3 {
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    let v = (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::new(0.5, 0.7, 1.0) * t );
    v
}

fn unit_vector(v : Vec3) -> Vec3 {
    v / v.length()
}

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;
    let mut file = File::create ("ch3.ppm")?;

    let first_line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write(first_line.as_bytes());

    let lower_corner = Vec3::new (-2.0, -1.0, -1.0);
    let horizontal = Vec3::new (5.0, 0.0, 0.0);
    let vertical = Vec3::new (0.0, 2.0, 0.0);
    let origin = Vec3::new (0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        //file.write(format!("{}\n", j).as_bytes());
        for i in 0..nx {
            let u = i as f32/ nx as f32;
            let v = j as f32/ ny as f32;

            let r = Ray::new (origin, lower_corner + horizontal * u + vertical * v);
            let col = color(r);

            let ir = 255.99 * col[0];
            let ig = 255.99 * col[1];
            let ib = 255.99 * col[2];
            file.write(format!("{} {} {}\n", ir as i32, ig as i32, ib as i32).as_bytes());
        }
    }

    Ok(())
}
