use crate::Ray;
use crate::Vec3;
use crate::HitRecord;

extern crate rand;
use rand::prelude::*;

// #[derive(Copy,Clone,Debug, Default)]
// pub struct HitRecord {
//     pub t : f32,
//     pub p : Vec3,
//     pub normal : Vec3
// }
#[derive(Copy,Clone,Debug)]
pub struct MaterialResult {
    pub attenuation : Vec3,
    pub scattered : Ray
}

pub trait Material{
    fn scatter(&self, r_in : Ray, hit_record : HitRecord) -> Option <MaterialResult>;
    fn box_clone(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material>
{
    fn clone(&self) -> Box<dyn Material> {
        self.box_clone()
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Lambertian {
    pub albedo : Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, r_in : Ray, rec : HitRecord) -> Option <MaterialResult> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let material_ray = MaterialResult { scattered : Ray::new (rec.p, target - rec.p), attenuation : self.albedo};
        return Some(material_ray);
    }
    fn box_clone(&self) -> Box<dyn Material> {
        return Box::new((*self).clone());
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Metal {
    pub albedo : Vec3,
}

impl Material for Metal {
    fn scatter(&self, r_in : Ray, rec : HitRecord) -> Option <MaterialResult> {
        let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new (rec.p, reflected);
        let material_ray = MaterialResult { scattered : scattered, attenuation : self.albedo};
        if scattered.direction().dot(rec.normal) > 0.0 {
            return Some(material_ray);
        }
        return None;
    }
    fn box_clone(&self) -> Box<dyn Material> {
        return Box::new((*self).clone());
    }
}

pub fn random_in_unit_sphere() -> Vec3{
    let mut rng = rand::thread_rng();
    loop {
        let mut r1: f32 = rng.gen();
        let mut r2: f32 = rng.gen();
        let mut r3: f32 = rng.gen();
        let p = Vec3::new(r1, r2, r3) * 2.0 - Vec3::new(1.0, 1.0, 1.0) ;
        if p.squared_length() < 1.0 {
            return p
        }
    }
}

pub fn reflect (v : Vec3, n : Vec3) -> Vec3 {
    return v - n*v.dot(n)*2.0;
}