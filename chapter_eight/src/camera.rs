use crate::Ray;
use crate::Vec3;

#[derive(Copy,Clone,Debug, Default)]
pub struct Camera {
    pub lower_left_corner : Vec3,
    pub horizontal : Vec3,
    pub vertical : Vec3,
    pub origin : Vec3,
}

impl Camera {
    pub fn new (lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov:f32, aspect:f32) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan(); 
        let half_width = aspect * half_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(vup.cross(w));
        let v = w.cross(u);

        dbg!(theta, half_height, half_width);
        dbg!(Vec3::unit_vector(lookfrom - lookat));

        dbg!(u, v, w);

        Camera {lower_left_corner : lookfrom - (u * half_width) - (v * half_height)  - w, 
            horizontal: u * half_width * 2.0, 
            vertical: v * half_height * 2.0,
            origin: lookfrom
        }
    }
    pub fn get_ray (&self, s:f32, t:f32) -> Ray {
        return Ray::new (self.origin, self.lower_left_corner + self.horizontal * s + self.vertical * t);
    }

}