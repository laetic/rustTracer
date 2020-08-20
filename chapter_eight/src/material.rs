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
    pub fuzz : f32,
}

impl Material for Metal {
    fn scatter(&self, r_in : Ray, rec : HitRecord) -> Option <MaterialResult> {
        let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new (rec.p, reflected + random_in_unit_sphere() * self.fuzz);
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

#[derive(Copy,Clone,Debug)]
pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {
    fn scatter(&self, r_in :Ray, rec : HitRecord) -> Option <MaterialResult> {
        let outward_normal : Vec3;
        let reflected = reflect(r_in.direction(), rec.normal);
        let ni_over_nt : f32;
        let attenuation = Vec3::new(1.0,1.0,1.0);
        let cosine : f32;
        let reflect_prob : f32;
        let mut scattered :Ray;
        let material_ray : MaterialResult;

        if r_in.direction().dot(rec.normal) > 0.0 { // into material
            outward_normal = rec.normal * -1.0;
            ni_over_nt = self.ref_idx;
            cosine = r_in.direction().dot(rec.normal) * self.ref_idx / r_in.direction().length();
        }
        else { // out of material
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = r_in.direction().dot(rec.normal) * -1.0 / r_in.direction().length();
        }
        let refracted = refract(r_in.direction(), outward_normal, ni_over_nt);
        match refracted {
            Some(x) => reflect_prob = schlick(cosine, self.ref_idx),
            None => reflect_prob = 1.0,
        }
        // if refracted.is_some() {
        //     reflect_prob = schlick(cosine, self.ref_idx);
        // }
        // else {
        //     scattered = Ray::new(rec.p, reflected);
        //     reflect_prob = 1.0;
        // }

        if rand::thread_rng().gen_range(0.0,1.0) < reflect_prob {
            scattered = Ray::new(rec.p, reflected);
            material_ray = MaterialResult { scattered : scattered, attenuation : attenuation};
            Some(material_ray)
        }
        else {
            scattered = Ray::new(rec.p, refracted.unwrap());
            material_ray = MaterialResult { scattered : scattered, attenuation : attenuation};
            Some(material_ray)
        }

    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}


pub fn random_in_unit_sphere() -> Vec3{
    loop {
        let r1: f32 = rand::thread_rng().gen_range(0.0,1.0);
        let r2: f32 = rand::thread_rng().gen_range(0.0,1.0);
        let r3: f32 = rand::thread_rng().gen_range(0.0,1.0);
        let p = Vec3::new(r1, r2, r3) * 2.0 - Vec3::new(1.0, 1.0, 1.0) ;
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn reflect (v : Vec3, n : Vec3) -> Vec3 {
    return v - n*v.dot(n)*2.0;
}

pub fn refract (v: Vec3, n : Vec3, ni_over_nt : f32) -> Option<Vec3> {
    let uv = Vec3::unit_vector(v);
    let dt = uv.dot(n);
    let refracted : Vec3;
    let discriminant = 1.0 - (ni_over_nt * ni_over_nt * (1.0 - (dt * dt)));
    if discriminant > 0.0 {
        refracted = ((uv - n*dt) * ni_over_nt ) - (n * discriminant.sqrt());
        return Some(refracted);
    }
    return None;
}

pub fn schlick (cosine : f32, ref_idx : f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}