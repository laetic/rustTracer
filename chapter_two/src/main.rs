mod vec3;
use vec3::*;
mod ray;
use ray::*;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut v1 = Vec3::new (1.0, 2.0, 3.0);
    let v2 = Vec3::new (4.0, 5.0, 6.0);
    println!("{}", v1);
    println!("addition: {} + {} = {}", v1, v2, (v1 + v2) );
    println!("indexes for v1: {}, {}, {}", v1[0], v1[1], v1[2]);
    
    println!("add assign: {}", v1);
    let v3 = v1 + v2;
    println!("adding v3 {} to v1", v3);
    v1 += v3;
    println!("result: {}", v1);

    v1 -= v3;
    println!("result: {}", v1);

    v1 *= 2.0;
    println!("result: {}", v1);

    v1 *= v2;
    println!("result: {}", v1);

    println!("length: {}", v1.length());

    v1.make_unit_vector();
    println!("result: {}", v1);

    println!("{} dot {} = {}", v1, v2, v1.dot(v2));

    println!("{} cross {} = {}", v1, v2, v1.cross(v2));

    let nx = 200;
    let ny = 100;
    let mut file = File::create ("ch2.ppm")?;
    let first_line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write(first_line.as_bytes());

    for j in (0..ny).rev() {
        //file.write(format!("{}\n", j).as_bytes());
        for i in 0..nx {
            let vec = Vec3::new ( i as f32 / nx as f32 , j as f32 / ny as f32, 0.2 );
            // let r = i as f32 / nx as f32;
            // let g = j as f32 / ny as f32;
            // let b = 0.2;
            let ir = 255.99 * vec[0];
            let ig = 255.99 * vec[1];
            let ib = 255.99 * vec[2];
            file.write(format!("{} {} {}\n", ir as i32, ig as i32, ib as i32).as_bytes());
        }
    }

    Ok(())
}
