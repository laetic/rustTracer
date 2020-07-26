mod vec3;
mod ray;
mod hitable;

use vec3::*;
use ray::*;
use hitable::*;
use std::fs::File;
use std::io::prelude::*;

fn hit_sphere (center: Vec3, radius: f32, r: Ray) -> f32 {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - (radius * radius);

    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        return -1.0;
    }
    else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn color(r: Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new (0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = unit_vector(r.point_at_parameter(t) - Vec3::new (0.0, 0.0, -1.0));
        return Vec3::new (N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) * 0.5;
    }
    
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
    let mut file = File::create ("ch4.ppm")?;

    let first_line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write(first_line.as_bytes());

    let lower_corner = Vec3::new (-2.0, -1.0, -1.0);
    let horizontal = Vec3::new (4.0, 0.0, 0.0);
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
