use super::bvh_node;
use super::camera;
use crate::hit::hittable::Hittable;
use crate::hit::record::HitRecord;
use crate::material::{self, diffuse_light, isotropic};
use crate::math;
use crate::object::{constant_medium, cubic, rect, rotate, sphere, translate};
use crate::texture::texturable;
use crate::texture::{checker_texture, solid_texture, image_texture};
use std::sync::Arc;
use crate::io::{picture, bitmap};

pub struct Scene {
    objects: Vec<Arc<dyn Hittable>>,
    bvh: bvh_node::BvhNode,
    camera: camera::Camera,
    backgound: math::vector::Color3,
}
impl Scene {
    pub fn new() -> Scene {
        // let mut objects = Scene::random_spheres();
        let mut ret = Scene::earth();
        let bvh = bvh_node::BvhNode::new(&mut ret.0, 0.0, 1.0);
        Scene {
            objects: ret.0,
            bvh: bvh,
            camera: ret.1,
            backgound: ret.2,
        }
    }
    fn random_spheres() -> (Vec<Arc<dyn Hittable>>, camera::Camera, math::vector::Color3) {
        let mut objects: Vec<Arc<dyn Hittable>> = vec![];
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(material::lambertian::Lambertian::new(
                checker_texture::CheckerTexture::new(
                    solid_texture::SolidTexture::new(math::vector::Color3::new(0.2, 0.3, 0.1)),
                    solid_texture::SolidTexture::new(math::vector::Color3::new(0.9, 0.9, 0.9)),
                ),
            )),
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
                            Arc::new(material::lambertian::Lambertian::new(
                                solid_texture::SolidTexture::new(albedo),
                            )),
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
                            Arc::new(material::metal::Metal::new(albedo, fuzz)),
                        )));
                    } else {
                        // glass
                        objects.push(Arc::new(sphere::Sphere::new(
                            center,
                            0.2,
                            Arc::new(material::dielectric::Dielectric::new(1.5)),
                        )));
                    }
                }
            }
        }

        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 1.0, 0.0),
            1.0,
            Arc::new(material::dielectric::Dielectric::new(1.5)),
        )));
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Arc::new(material::lambertian::Lambertian::new(
                solid_texture::SolidTexture::new(math::vector::Color3::new(0.4, 0.2, 0.1)),
            )),
        )));
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(4.0, 1.0, 0.0),
            1.0,
            Arc::new(material::metal::Metal::new(
                math::vector::Color3::new(0.7, 0.6, 0.5),
                0.0,
            )),
        )));
        let look_from = math::vector::Point3::new(13.0, 2.0, 3.0);
        let look_at = math::vector::Point3::new(0.0, 0.0, 0.0);
        let focus_dist = 10.0;
        let cam = camera::Camera::new(
            look_from,
            look_at,
            math::vector::Dir3::new(0.0, 1.0, 0.0),
            20.0,
            0.1,
            focus_dist,
            0.0,
            1.0,
        );
        let backgound = math::vector::Color3::new(0.7, 0.8, 1.0);
        (objects, cam, backgound)
    }
    fn earth() -> (Vec<Arc<dyn Hittable>>, camera::Camera, math::vector::Color3) {
        let mut pic = picture::Jpeg::new("asset/earthmap.jpg".to_string());
        let mut bitmap = bitmap::Bitmap::new_empty();
        let pixels = pic.load(&mut bitmap);

        let mut objects: Vec<Arc<dyn Hittable>> = vec![];
        let earth_surface = image_texture::ImageTexture::new(Arc::new(bitmap));
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 0.0, 0.0),
            2.0,
            Arc::new(material::lambertian::Lambertian::new(earth_surface)),
        )));
    
        let look_from = math::vector::Point3::new(13.0, 2.0, 3.0);
        let look_at = math::vector::Point3::new(0.0, 0.0, 0.0);
        let focus_dist = 10.0;
        let cam = camera::Camera::new(
            look_from,
            look_at,
            math::vector::Dir3::new(0.0, 1.0, 0.0),
            20.0,
            0.1,
            focus_dist,
            0.0,
            1.0,
        );
        let backgound = math::vector::Color3::new(0.7, 0.8, 1.0);
        (objects, cam, backgound)
    }
    fn two_spheres() -> (Vec<Arc<dyn Hittable>>, camera::Camera, math::vector::Color3) {
        let mut objects: Vec<Arc<dyn Hittable>> = vec![];
        let checker = checker_texture::CheckerTexture::new(
            solid_texture::SolidTexture::new(math::vector::Color3::new(0.2, 0.3, 0.1)),
            solid_texture::SolidTexture::new(math::vector::Color3::new(0.9, 0.9, 0.9)),
        );
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, -10.0, 0.0),
            10.0,
            Arc::new(material::lambertian::Lambertian::new(checker)),
        )));
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 10.0, 0.0),
            10.0,
            Arc::new(material::lambertian::Lambertian::new(checker)),
        )));
        let look_from = math::vector::Point3::new(13.0, 2.0, 3.0);
        let look_at = math::vector::Point3::new(0.0, 0.0, 0.0);
        let focus_dist = 10.0;
        let cam = camera::Camera::new(
            look_from,
            look_at,
            math::vector::Dir3::new(0.0, 1.0, 0.0),
            20.0,
            0.1,
            focus_dist,
            0.0,
            1.0,
        );
        let backgound = math::vector::Color3::new(0.7, 0.8, 1.0);
        (objects, cam, backgound)
    }
    fn simple_light() -> (Vec<Arc<dyn Hittable>>, camera::Camera, math::vector::Color3) {
        let mut objects: Vec<Arc<dyn Hittable>> = vec![];
        let red = Arc::new(material::lambertian::Lambertian::new(
            solid_texture::SolidTexture::new(math::vector::Color3::new(0.65, 0.05, 0.05)),
        ));
        let white = Arc::new(material::lambertian::Lambertian::new(
            solid_texture::SolidTexture::new(math::vector::Color3::new(0.73, 0.73, 0.73)),
        ));
        let green = Arc::new(material::lambertian::Lambertian::new(
            solid_texture::SolidTexture::new(math::vector::Color3::new(0.12, 0.45, 0.15)),
        ));
        let checker = checker_texture::CheckerTexture::new(
            solid_texture::SolidTexture::new(math::vector::Color3::new(0.2, 0.3, 0.1)),
            solid_texture::SolidTexture::new(math::vector::Color3::new(0.9, 0.9, 0.9)),
        );
        // objects.push(Arc::new(sphere::Sphere::new(
        //     math::vector::Point3::new(0.0, -1000.0, 0.0),
        //     1000.0,
        //     material::lambertian::Lambertian::new(checker),
        // )));
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 2.0, -0.0),
            2.0,
            Arc::new(material::lambertian::Lambertian::new(checker)),
        )));
        let lightcolor =
            solid_texture::SolidTexture::new(math::vector::Color3::new(15.0, 15.0, 15.0));
        let difflight = Arc::new(diffuse_light::DiffuseLight::new(lightcolor));
        objects.push(Arc::new(rect::XYRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));
        objects.push(Arc::new(rect::XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, red)));
        objects.push(Arc::new(rect::YZRect::new(3.0, 5.0, 1.0, 3.0, -2.0, white)));

        objects.push(Arc::new(rect::XZRect::new(3.0, 5.0, 1.0, 3.0, -1.0, green)));
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 5.0, -2.0),
            1.0,
            difflight,
        )));
        let look_from = math::vector::Point3::new(0.0, 0.0, -60.0);
        let look_at = math::vector::Point3::new(0.0, 2.0, 0.0);
        let focus_dist = 500.0;
        let cam = camera::Camera::new(
            look_from,
            look_at,
            math::vector::Dir3::new(0.0, 2.0, 0.0),
            20.0,
            0.1,
            focus_dist,
            0.0,
            1.0,
        );
        (objects, cam, math::vector::Color3::new(0.0, 0.0, 0.0))
    }
    fn cornell_box() -> (Vec<Arc<dyn Hittable>>, camera::Camera, math::vector::Color3) {
        let mut objects: Vec<Arc<dyn Hittable>> = vec![];
        let red = Arc::new(material::lambertian::Lambertian::new(
            solid_texture::SolidTexture::new(math::vector::Color3::new(0.65, 0.05, 0.05)),
        ));
        let white = Arc::new(material::lambertian::Lambertian::new(
            solid_texture::SolidTexture::new(math::vector::Color3::new(0.73, 0.73, 0.73)),
        ));
        let green = Arc::new(material::lambertian::Lambertian::new(
            solid_texture::SolidTexture::new(math::vector::Color3::new(0.12, 0.45, 0.15)),
        ));
        let lightcolor =
            solid_texture::SolidTexture::new(math::vector::Color3::new(15.0, 15.0, 15.0));
        let difflight = Arc::new(diffuse_light::DiffuseLight::new(lightcolor));
        objects.push(Arc::new(rect::YZRect::new(
            0.0, 555.0, 0.0, 555.0, 555.0, green,
        )));
        objects.push(Arc::new(rect::YZRect::new(
            0.0, 555.0, 0.0, 555.0, 0.0, red,
        )));
        objects.push(Arc::new(rect::XZRect::new(
            213.0, 343.0, 227.0, 332.0, 554.0, difflight,
        )));
        objects.push(Arc::new(rect::XZRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            white.clone(),
        )));
        objects.push(Arc::new(rect::XZRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));
        objects.push(Arc::new(rect::XYRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));

        objects.push(Arc::new(constant_medium::ConstantMedium::new(
            Arc::new(translate::Translate::new(
                Arc::new(rotate::RotateY::new(
                    Arc::new(cubic::Cubic::new(
                        math::vector::Point3::new(0.0, 0.0, 0.0),
                        math::vector::Point3::new(165.0, 330.0, 165.0),
                        white.clone(),
                    )),
                    15.0,
                )),
                math::vector::Dir3::new(265.0, 0.0, 295.0),
            )),
            0.01,
            Arc::new(isotropic::Isotropic::new(solid_texture::SolidTexture::new(
                math::vector::Color3::zero(),
            ))),
        )));
        objects.push(Arc::new(constant_medium::ConstantMedium::new(
            Arc::new(translate::Translate::new(
                Arc::new(rotate::RotateY::new(
                    Arc::new(cubic::Cubic::new(
                        math::vector::Point3::new(0.0, 0.0, 0.0),
                        math::vector::Point3::new(165.0, 165.0, 165.0),
                        white.clone(),
                    )),
                    -18.0,
                )),
                math::vector::Dir3::new(130.0, 0.0, 65.0),
            )),
            0.01,
            Arc::new(isotropic::Isotropic::new(solid_texture::SolidTexture::new(
                math::vector::Color3::one(),
            ))),
        )));

        let look_from = math::vector::Point3::new(278.0, 278.0, -800.0);
        let look_at = math::vector::Point3::new(278.0, 278.0, 0.0);
        let focus_dist = 50.0;
        let cam = camera::Camera::new(
            look_from,
            look_at,
            math::vector::Dir3::new(0.0, 2.0, 0.0),
            40.0,
            0.1,
            focus_dist,
            0.0,
            1.0,
        );
        (objects, cam, math::vector::Color3::new(0.0, 0.0, 0.0))
    }
    pub fn hit(
        &self,
        ray: &math::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> std::option::Option<HitRecord> {
        self.bvh.hit(ray, t_min, t_max)
    }
    // pub fn hit(&self, ray: &math::ray::Ray, t_min: f64, t_max: f64) -> std::option::Option<HitRecord> {
    //     let mut hit_anything = std::option::Option::<HitRecord>::None;
    //     let mut closest_so_far = t_max;
    //     for obj in &self.objects {
    //         let hit = obj.hit(&ray, t_min, closest_so_far);
    //         match hit {
    //             Option::Some(r) => {
    //                 closest_so_far = r.t();
    //                 hit_anything = Option::Some(r);
    //             }
    //             Option::None => {}
    //         }
    //     }
    //     return hit_anything;
    // }
    pub fn camera(&self) -> &camera::Camera {
        &self.camera
    }
    pub fn background(&self) -> &math::vector::Color3 {
        &self.backgound
    }
}
