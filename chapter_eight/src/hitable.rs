use crate::Ray;
use crate::Vec3;
use crate::material::*;

#[derive(Clone)]
pub struct HitRecord {
    pub t : f32,
    pub p : Vec3,
    pub normal : Vec3,
    pub material : Box <dyn Material>,
}

pub trait Hitable {
    fn hit (&self, r : Ray, t_min: f32, t_max: f32) -> Option <HitRecord>;
}