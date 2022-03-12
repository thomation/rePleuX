use crate::hit;
use crate::math::ray;

pub struct HittableList {
    objects: Vec<Box<hit::Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }
    pub fn add(&mut self, obj: Box<hit::Hittable>) {
        self.objects.push(obj);
    }
}

impl hit::Hittable for HittableList {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> std::option::Option<hit::HitRecord> {
        let mut hit_anything = std::option::Option::<hit::HitRecord>::None;
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
