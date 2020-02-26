#[derive(Debug)]
pub struct Vec3 {
    pub e : [f32; 3]
}

impl Vec3 {
    pub fn new(x: f32, y:f32, z:f32) -> Vec3 {
        Vec3 {e: [x, y, z]}
    }

    pub fn x(&self) -> f32 { self.e[0] }
    pub fn y(&self) -> f32 { self.e[1] }
    pub fn z(&self) -> f32 { self.e[2] }
    // pub fn r(&self) -> f32 { self.e[0] }
    // pub fn g(&self) -> f32 { self.e[1] }
    // pub fn b(&self) -> f32 { self.e[2] }
    pub fn to_string(&self) -> String {
        format! ( "{} {} {}", self.x(), self.y(), self.z())
    }
}