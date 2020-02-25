use vec3;

fn main() {
    let v1 = vec3::new(1, 2, 3);
    let x : String = v1[0].to_string();
    let y : String = v1[1].to_string();
    let z : String = v1[2].to_string();
    println! ("x: {}, y: {}, z: {}", x, y, z);
}
