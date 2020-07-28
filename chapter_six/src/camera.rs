use crate::Ray;
use crate::Vec3;

pub struct Camera {
    pub lower_corner : Vec3,
    pub horizontal : Vec3,
    pub vertical : Vec3,
    pub origin : Vec3,
}

impl Camera {
    pub fn get_ray (&self, u:f32, v:f32) -> Ray {
        return Ray::new (self.origin, self.lower_corner + self.horizontal * u + self.vertical * v);
    }
}