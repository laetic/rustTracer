use crate::hitable::*;
use crate::Ray;

pub struct HitableList {
    pub list : Vec<Box<dyn Hitable>>,
    pub list_size: i32
}

impl HitableList {
    pub fn new (l: Vec<Box<dyn Hitable>>, n: i32 ) -> HitableList {
        HitableList {list:l, list_size:n}
    }
}

impl Hitable for HitableList {
    fn hit (&self, r : Ray, t_min: f32, t_max: f32, mut rec : HitRecord) -> bool {
        let temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for i in 0..self.list_size {
            if self.list[i as usize].hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec
            }
        }
        return hit_anything;
    }
}