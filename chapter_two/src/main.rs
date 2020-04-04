mod vec3;

fn main() {
    let v1 = vec3::Vec3::new (1.0, 2.0, 3.0);
    let v2 = vec3::Vec3::new (4.0, 5.0, 6.0);
    println!("{}", v1.to_string());
    println!("addition: {} + {} = {}", v1.to_string(), v2.to_string(), (v1 + v2).to_string() );
}
