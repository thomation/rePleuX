use crate::hit::hittable::Hittable;
use crate::hit::record::HitRecord;
use crate::material;
use crate::math;
use crate::object::sphere;

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
}
impl Scene {
    pub fn new() -> Scene {
        let mut objects: Vec<Box<dyn Hittable>> = vec![];
        objects.push(Box::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            material::lambertian::Lambertian::new(math::vector::Color3::new(0.5, 0.5, 0.5)),
        )));

        // Random spheres

        objects.push(Box::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 1.0, 0.0),
            1.0,
            material::dielectric::Dielectric::new(1.5),
        )));
        objects.push(Box::new(sphere::Sphere::new(
            math::vector::Point3::new(-4.0, 1.0, 0.0),
            1.0,
            material::lambertian::Lambertian::new(math::vector::Color3::new(0.4, 0.2, 0.1)),
        )));
        objects.push(Box::new(sphere::Sphere::new(
            math::vector::Point3::new(4.0, 1.0, 0.0),
            1.0,
            material::metal::Metal::new(math::vector::Color3::new(0.7, 0.6, 0.5), 0.0),
        )));

        Scene { objects: objects }
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
                Option::None => {}
            }
        }
        return hit_anything;
    }
}
