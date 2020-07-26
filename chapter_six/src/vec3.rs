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
    pub fn length(&self) -> f32 { (self.x()*self.x() + self.y()*self.y() + self.z()*self.z()).sqrt()}
    pub fn squared_length(&self) -> f32 {(self.x()*self.x() + self.y()*self.y() + self.z()*self.z())}
    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k; self.e[1] *= k; self.e[2] *= k;
    }
    pub fn dot(&self, other:Vec3) -> f32 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other:Vec3) -> Vec3 {
        Vec3 { e: [ self.e[1] * other.e[2] - self.e[2] * other.e[1], 
            -( self.e[0] * other.e[2] - self.e[2] * other.e[0] ), 
            self.e[0] * other.e[1] - self.e[1] * other.e[0]]}
    }
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


impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other:Vec3) -> Vec3 {
        Vec3 { e: [ self.e[0] * other.e[0],  self.e[1] * other.e[1],  self.e[2] * other.e[2] ] }
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other:Vec3) -> Vec3 {
        Vec3 { e: [ self.e[0] / other.e[0],  self.e[1] / other.e[1],  self.e[2] / other.e[2] ] }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other:f32) -> Vec3 {
        Vec3 { e: [ self.e[0] * other,  self.e[1] * other,  self.e[2] * other ] }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other:f32) -> Vec3 {
        Vec3 { e: [ self.e[0] / other,  self.e[1] / other,  self.e[2] / other ] }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        &self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [ self.e[0] + other.e[0],  self.e[1] + other.e[1],  self.e[2] + other.e[2] ]
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            e: [ self.e[0] - other.e[0],  self.e[1] - other.e[1],  self.e[2] - other.e[2] ]
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            e: [ self.e[0] * other.e[0],  self.e[1] * other.e[1],  self.e[2] * other.e[2] ]
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other:f32) {
        *self = Self {
            e: [ self.e[0] * other ,  self.e[1] * other ,  self.e[2] * other  ]
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            e: [ self.e[0] / other.e[0],  self.e[1] / other.e[1],  self.e[2] / other.e[2] ]
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other:f32) {
        *self = Self {
            e: [ self.e[0] / other ,  self.e[1] / other ,  self.e[2] / other  ]
        }
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