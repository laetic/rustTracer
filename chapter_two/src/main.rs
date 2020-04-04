mod vec3;

fn main() {
    let mut v1 = vec3::Vec3::new (1.0, 2.0, 3.0);
    let v2 = vec3::Vec3::new (4.0, 5.0, 6.0);
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
}
