use crate::hitable::*;
use crate::Vec3;
use crate::Ray;

#[derive(Copy,Clone,Debug)]
pub struct Sphere {
    pub center : Vec3,
    pub radius : f32
}

impl Sphere {
    pub fn new (cen: Vec3, r: f32 ) -> Sphere {
        Sphere {center:cen, radius:r}
    }
}

impl Hitable for Sphere {
    fn hit (&self, r : Ray, t_min: f32, t_max: f32) -> Option {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = 2.0 * oc.dot(r.direction());
        let c = oc.dot(oc) - (self.radius * self.radius);
    
        let discriminant = b*b - 4.0*a*c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b*b - a*c).sqrt()) / a;
            if (temp < t_max && temp > t_min) {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }

            temp = (-b - (b*b + a*c).sqrt()) / a;
            if (temp < t_max && temp > t_min) {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}