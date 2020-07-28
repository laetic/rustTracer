use crate::hitable::*;
use crate::Ray;

pub struct HitableList {
    pub list : Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new (l: Vec<Box<dyn Hitable>>, n: i32 ) -> HitableList {
        HitableList {list:l}
    }
}

impl Hitable for HitableList {
    fn hit (&self, r : Ray, t_min: f32, t_max: f32 ) -> Option <HitRecord>{
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        let hit = self.list[1].hit(r, t_min, closest_so_far);

        for item in self.list.iter() {
            if let Some(hit) = item.hit(r, t_min, closest_so_far){
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }

        }
        hit_anything
    }
}