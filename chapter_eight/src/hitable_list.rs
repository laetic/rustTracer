use crate::hitable::*;
use crate::Ray;
use crate::Sphere;
use crate::Vec3;
use crate::material::*;
use crate::rand::*;

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

        //let hit = self.list[1].hit(r, t_min, closest_so_far);

        for item in self.list.iter() {
            if let Some(hit) = item.hit(r, t_min, closest_so_far){
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }

        }
        hit_anything
    }
}

pub fn random_scene () -> HitableList{
    let mut list: Vec<Box<dyn Hitable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielectric{ref_idx : 1.5}))));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0,0.0), 1000.0, Box::new(Lambertian{albedo: Vec3::new(0.5, 0.5, 0.5)}))));
    list.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal{albedo: Vec3::new(0.2, 0.4, 0.8), fuzz: 0.0}))));
    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian{albedo: Vec3::new(0.8, 0.3, 0.3)}))));

    let n = 11;
    for a in -n..n {
        for b in -n..n {
            let mut rng = rand::thread_rng();
            let choose_mat : f32 = rng.gen();
            let center = Vec3::new (a as f32 + (0.9 * rng.gen::<f32>()), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 { 
                if choose_mat < 0.8 { // diffuse
                    list.push(Box::new(Sphere::new(center, 0.2, Box::new(Lambertian{albedo: Vec3::new(rng.gen::<f32>()*rng.gen::<f32>(), rng.gen::<f32>()*rng.gen::<f32>(), rng.gen::<f32>()*rng.gen::<f32>())}))));
                }
                else if choose_mat < 0.95 { //metal
                    list.push(Box::new(Sphere::new(center, 0.2, Box::new(Metal{albedo: Vec3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 1.0 + rng.gen::<f32>()), fuzz: 0.5 *  rng.gen::<f32>()}))));
                }
                else {
                    list.push(Box::new(Sphere::new(center, 0.2, Box::new(Dielectric{ref_idx : 1.5}))));
                }
            }
        }
    }

    let world = HitableList::new(list, 2);
    world
}