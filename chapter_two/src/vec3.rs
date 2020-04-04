use std::ops::*;
use std::fmt;

#[derive(Copy,Clone,Debug)]
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
    pub fn r(&self) -> f32 { self.e[0] }
    pub fn g(&self) -> f32 { self.e[1] }
    pub fn b(&self) -> f32 { self.e[2] }

    // pub fn to_string(&self) -> String {
    //     format! ( "({},{},{})", self.x(), self.y(), self.z())
    // }
}

// https://doc.rust-lang.org/1.30.0/book/2018-edition/appendix-02-operators.html?highlight=op#operators
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other:Vec3 ) -> Vec3 {
        Vec3 { e : [ self.e[0] + other.e[0],  self.e[1] + other.e[1],  self.e[2] + other.e[2] ]}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other:Vec3 ) -> Vec3 {
        Vec3 { e : [ self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2] ]}
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        &self.e[index]
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[1])
    }
}

//impl Copy for Vec3 { }

// impl Clone for Vec3 {
//     fn clone(&self) -> Vec3 {
//         *self
//     }
// }