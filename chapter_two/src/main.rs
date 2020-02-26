mod Vec3;

fn main() {
    let v1 = Vec3::Vec3::new (1.0, 2.0, 3.0);
    let v2 = Vec3::Vec3 {e:[1.0,2.0,3.0]};

    println!("{}", v1.to_string())
}
