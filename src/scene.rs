use crate::hit::hittable::Hittable;
use crate::hit::record::HitRecord;
use crate::object::sphere;
use crate::math;
use crate::material;

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
}
impl Scene {
    pub fn new() -> Scene {
        let material_ground = material::lambertian::Lambertian::new(math::vector::Color3::new(0.8, 0.8, 0.0));
        let material_center = material::lambertian::Lambertian::new(math::vector::Color3::new(0.7, 0.3, 0.3));
        let material_left = material::metal::Metal::new(math::vector::Color3::new(0.8, 0.8, 0.8), 0.3);
        let material_right = material::metal::Metal::new(math::vector::Color3::new(0.8, 0.6, 0.2), 1.0);

        let mut objects : Vec<Box<dyn Hittable>> = vec![];
        objects.push(Box::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground
        )));
        objects.push(Box::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 0.0, -1.0),
            0.5,
            material_center
        )));
        objects.push(Box::new(sphere::Sphere::new(
            math::vector::Point3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left
        )));
        objects.push(Box::new(sphere::Sphere::new(
            math::vector::Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right
        )));
        Scene {
            objects: objects,
        }
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &math::ray::Ray, t_min: f64, t_max: f64) -> std::option::Option<HitRecord> {
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