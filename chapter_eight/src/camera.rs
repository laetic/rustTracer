use crate::Ray;
use crate::Vec3;

pub struct Camera {
    pub lower_left_corner : Vec3,
    pub horizontal : Vec3,
    pub vertical : Vec3,
    pub origin : Vec3,
}

impl Camera {
    pub fn new (vfov:f32, aspect:f32) -> Camera {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = (aspect * half_height);
        return Camera {lower_left_corner : Vec3::new(-half_width, -half_height, -1.0), 
            horizontal: Vec3::new(2.0 * half_width, 0.0, 0.0), 
            vertical: Vec3::new (0.0, 2.0 * half_height, 0.0),
            origin: Vec3::new (0.0, 0.0, 0.0)}
    }
    pub fn get_ray (&self, u:f32, v:f32) -> Ray {
        return Ray::new (self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v);
    }

}