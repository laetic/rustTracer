use crate::Hitable;
use crate::Vec3;

pub struct Sphere {
    pub center : Vec3;
    pub radius : f32;
}

impl Sphere {
    pub fn new (cen: Vec3, r: f32 ) -> Sphere {
        Sphere {center:cen, radius:r}
    }
}

impl Hitable for Sphere {
    type Output = bool;
    fn hit (&self, r : Ray, t_min: f32, t_max: f32, rec : HitRecord) {
        let oc = r.origin() - center;
        let a = r.direction().dot(r.direction());
        let b = 2.0 * oc.dot(r.direction());
        let c = oc.dot(oc) - (radius * radius);
    
        let discriminant = b*b - 4.0*a*c;
        if discriminant > 0.0 {
            let temp = (-b - (b*b - a*c).sqrt()) / a;
            if (temp < t_max && temp > t_min) {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t)
                rec.normal = (rec.p - center) / radius;
                return true;
            }

            temp = (-b - (b*b + a*c).sqrt()) / a;
            if (temp < t_max && temp > t_min) {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t)
                rec.normal = (rec.p - center) / radius;
                return true;
            }
        }
        return false;
    }
}