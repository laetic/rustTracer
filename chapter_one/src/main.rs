use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;
    let mut file = File::create ("ppm.ppm")?;
    let first_line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write(first_line.as_bytes());

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2;
            let ir = 255.99 * r;
            let ig = 255.99 * g;
            let ib = 255.99 * b;
            file.write(format!("{} {} {}\n", ir as i32, ig as i32, ib as i32).as_bytes());
        }
    }

    //file.write(b"Hello World!")?;
    Ok(())
}