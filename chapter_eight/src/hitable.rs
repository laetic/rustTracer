use crate::Ray;
use crate::Vec3;
use crate::material::*;

pub struct HitRecord <'a>{
    pub t : f32,
    pub p : Vec3,
    pub normal : Vec3,
    pub material : &'a dyn Material,
}

pub trait Hitable {
    fn hit (&self, r : Ray, t_min: f32, t_max: f32) -> Option <HitRecord>;
}