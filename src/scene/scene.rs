use super::bvh_node;
use crate::hit::hittable::Hittable;
use crate::hit::record::HitRecord;
use crate::material;
use crate::math;
use crate::object::sphere;
use crate::texture::solid_texture;
use std::sync::Arc;

pub struct Scene {
    bvh: bvh_node::BvhNode,
}
impl Scene {
    pub fn new() -> Scene {
        let mut objects: Vec<Arc<dyn Hittable>> = vec![];
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            material::lambertian::Lambertian::new(solid_texture::SolidTexture::new(math::vector::Color3::new(0.5, 0.5, 0.5))),
        )));

        // Random spheres
        let point = math::vector::Point3::new(4.0, 0.2, 0.0);
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = math::random::generate();
                let center = math::vector::Point3::new(
                    a as f64 + 0.9 * math::random::generate(),
                    0.2,
                    b as f64 + math::random::generate(),
                );
                if (center.clone() - &point).length() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo =
                            math::vector::Color3::random() * math::vector::Color3::random();
                        let center2 = center
                            + math::vector::Vec3::new(
                                0.0,
                                math::random::generate_range(0.0, 0.5),
                                0.0,
                            );
                        objects.push(Arc::new(sphere::Sphere::new_move(
                            center,
                            center2,
                            0.2,
                            material::lambertian::Lambertian::new(solid_texture::SolidTexture::new(albedo)),
                            0.0,
                            1.0,
                        )));
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = math::vector::Color3::random_range(0.5, 1.0);
                        let fuzz = math::random::generate_range(0.0, 0.5);
                        objects.push(Arc::new(sphere::Sphere::new(
                            center,
                            0.2,
                            material::metal::Metal::new(albedo, fuzz),
                        )));
                    } else {
                        // glass
                        objects.push(Arc::new(sphere::Sphere::new(
                            center,
                            0.2,
                            material::dielectric::Dielectric::new(1.5),
                        )));
                    }
                }
            }
        }

        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 1.0, 0.0),
            1.0,
            material::dielectric::Dielectric::new(1.5),
        )));
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(-4.0, 1.0, 0.0),
            1.0,
            material::lambertian::Lambertian::new(solid_texture::SolidTexture::new(math::vector::Color3::new(0.4, 0.2, 0.1))),
        )));
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(4.0, 1.0, 0.0),
            1.0,
            material::metal::Metal::new(math::vector::Color3::new(0.7, 0.6, 0.5), 0.0),
        )));

        let bvh = bvh_node::BvhNode::new(&mut objects, 0.0, 1.0);
        Scene {
            bvh: bvh,
        }
    }
    pub fn hit(&self, ray: &math::ray::Ray, t_min: f64, t_max: f64) -> std::option::Option<HitRecord> {
        self.bvh.hit(ray, t_min, t_max)
    }
}
