use crate::Ray;
use crate::Vec3;
use crate::rand::*;

#[derive(Copy,Clone,Debug, Default)]
pub struct Camera {
    pub lower_left_corner : Vec3,
    pub horizontal : Vec3,
    pub vertical : Vec3,
    pub origin : Vec3,
    pub lens_radius: f32,
    pub u : Vec3,
    pub v : Vec3,
    pub w : Vec3,
}

impl Camera {
    pub fn new (lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov:f32, aspect:f32, aperture:f32, focus_dist:f32) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan(); 
        let half_width = aspect * half_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(vup.cross(w));
        let v = w.cross(u);

        dbg!(theta, half_height, half_width);
        dbg!(Vec3::unit_vector(lookfrom - lookat));

        dbg!(u, v, w);

        Camera {lower_left_corner : lookfrom - (u * half_width * focus_dist) - (v * half_height * focus_dist)  - (w * focus_dist), 
            horizontal: u * half_width * focus_dist * 2.0, 
            vertical: v * half_height * focus_dist * 2.0,
            origin: lookfrom,
            lens_radius: aperture / 2.0,
            u : u,
            v : v,
            w : w,
        }
    }
    pub fn get_ray (&self, s:f32, t:f32) -> Ray {
        let rd =  random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x()  + self.v * rd.y();
        return Ray::new (self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset);
    }

}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p : Vec3;
    loop  {
        p = ( Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0,1.0,0.0) ) * 2.0;
        //dbg!(p);
        if p.dot(p) < 1.0 {
            break;
        }
    }
    p
}