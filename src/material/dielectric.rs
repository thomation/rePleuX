use super::material;
use super::scatter;
use crate::hit;
use crate::math;
pub struct Dielectric {
    index_of_fraction: f64,
}
impl Dielectric {
    pub fn new(index_of_fraction: f64) -> Dielectric {
        Dielectric {
            index_of_fraction: index_of_fraction,
        }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        let c = 1.0 - cosine;
        r0 + (1.0 - r0) * c.powf(5.0)
    }
}
impl material::Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &math::ray::Ray,
        hit_record: &hit::record::HitRecord,
    ) -> std::option::Option<scatter::ScatterResult> {
        let refraction_ratio = if hit_record.front_face() {
            1.0 / self.index_of_fraction
        } else {
            self.index_of_fraction
        };
        let normal = hit_record.normal();
        let unit_dir = math::vector::Vec3::unit(&ray_in.dir());
        let cos_theta =
            math::vector::Vec3::dot(&math::vector::Vec3::neg(&unit_dir), &normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if refraction_ratio * sin_theta > 1.0  || Dielectric::reflectance(cos_theta, refraction_ratio) > math::random::generate_range(0.0, 1.0) {
            let reflected = math::vector::Vec3::reflect(&unit_dir, &normal);
            Option::Some(scatter::ScatterResult::new(
                math::ray::Ray::new(hit_record.position().clone(), reflected),
                math::vector::Color3::new(1.0, 1.0, 1.0),
            ))
        } else {
            let refracted = math::vector::Vec3::refract(&unit_dir, &normal, refraction_ratio);
            Option::Some(scatter::ScatterResult::new(
                math::ray::Ray::new(hit_record.position().clone(), refracted),
                math::vector::Color3::new(1.0, 1.0, 1.0),
            ))
        }
    }
}
