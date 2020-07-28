use crate::hitable::*;
use crate::Vec3;
use crate::Ray;
use crate::Material;


pub struct Sphere <'b>{ 
    pub center : Vec3,
    pub radius : f32,
    pub material : &'b dyn Material,
}

impl Sphere <'_>{
    pub fn new (cen: Vec3, r: f32 , m:  &'b dyn Material) -> Sphere <'b>{
        Sphere {center:cen, radius:r, material : m}
    }
}

impl Hitable for Sphere <'_>{
    fn hit (&self, r : Ray, t_min: f32, t_max: f32) -> Option <HitRecord>{
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - (self.radius * self.radius);
    
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b*b - a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let pap = r.point_at_parameter(temp);
                let norm = (pap - self.center) / self.radius;
                return Some(HitRecord{t: temp, p: pap, normal: norm, material: self.material});
            }

            temp = (-b - (b*b + a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let pap = r.point_at_parameter(temp);
                let norm = (pap - self.center) / self.radius;
                return Some(HitRecord{t: temp, p: pap, normal: norm, material: self.material});
            }
        }
        return None;
    }
}