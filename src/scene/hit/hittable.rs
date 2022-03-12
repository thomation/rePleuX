use super::record::HitRecord;
use crate::math::ray;

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> std::option::Option<HitRecord> {
        let mut hit_anything = std::option::Option::<HitRecord>::None;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            let hit = obj.hit(&ray, t_min, closest_so_far);
            match hit {
                Option::Some(r) => {
                    closest_so_far = r.t();
                    hit_anything = Option::Some(r);
                }
                Option::None => {

                }
            }
        }
        return hit_anything;
    }
}
