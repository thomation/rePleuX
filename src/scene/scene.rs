use super::bvh_node;
use super::camera;
use crate::hit::{hittable::Hittable, hittable_list::HittableList, record::HitRecord};
use crate::io::{bitmap, picture};
use crate::material::lambertian;
use crate::material::{self, diffuse_light, isotropic};
use crate::math::{self, random, vector};
use crate::object::flip_face::FlipFace;
use crate::object::{constant_medium, cubic, flip_face, rect, rotate, sphere, translate};
use crate::texture::{checker_texture, image_texture, noise_texture, solid_texture};
use std::sync::Arc;

pub struct Scene {
    objects: Vec<Arc<dyn Hittable>>,
    bvh: bvh_node::BvhNode,
    camera: camera::Camera,
    backgound: math::vector::Color3,
    lights: Arc<dyn Hittable>,
}
impl Scene {
    pub fn new() -> Scene {
        // let mut objects = Scene::random_spheres();
        let mut ret = Scene::cornell_box();
        let bvh = bvh_node::BvhNode::new(&mut ret.0, 0.0, 1.0);
        Scene {
            objects: ret.0,
            lights: ret.1,
            bvh: bvh,
            camera: ret.2,
            backgound: ret.3,
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
                let choose_mat = random::generate();
                let center = vector::Point3::new(
                    a as f64 + 0.9 * random::generate(),
                    0.2,
                    b as f64 + random::generate(),
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
        pic.load(&mut bitmap);

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

    fn two_perlin_spheres() -> (Vec<Arc<dyn Hittable>>, camera::Camera, math::vector::Color3) {
        let mut objects: Vec<Arc<dyn Hittable>> = vec![];
        let perlintexure = noise_texture::NoiseTexture::new(4.0);
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(material::lambertian::Lambertian::new(perlintexure.clone())),
        )));
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 2.0, 0.0),
            2.0,
            Arc::new(material::lambertian::Lambertian::new(perlintexure)),
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
    fn cornell_box() -> (
        Vec<Arc<dyn Hittable>>,
        Arc<dyn Hittable>,
        camera::Camera,
        math::vector::Color3,
    ) {
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
        objects.push(Arc::new(FlipFace::new(Arc::new(rect::XZRect::new(
            213.0, 343.0, 227.0, 332.0, 554.0, difflight.clone(),
        )))));

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

        // objects.push(Arc::new(constant_medium::ConstantMedium::new(
        //     Arc::new(translate::Translate::new(
        //         Arc::new(rotate::RotateY::new(
        //             Arc::new(cubic::Cubic::new(
        //                 math::vector::Point3::new(0.0, 0.0, 0.0),
        //                 math::vector::Point3::new(165.0, 330.0, 165.0),
        //                 white.clone(),
        //             )),
        //             15.0,
        //         )),
        //         math::vector::Dir3::new(265.0, 0.0, 295.0),
        //     )),
        //     0.01,
        //     Arc::new(isotropic::Isotropic::new(solid_texture::SolidTexture::new(
        //         math::vector::Color3::zero(),
        //     ))),
        // )));
        // objects.push(Arc::new(constant_medium::ConstantMedium::new(
        //     Arc::new(translate::Translate::new(
        //         Arc::new(rotate::RotateY::new(
        //             Arc::new(cubic::Cubic::new(
        //                 math::vector::Point3::new(0.0, 0.0, 0.0),
        //                 math::vector::Point3::new(165.0, 165.0, 165.0),
        //                 white.clone(),
        //             )),
        //             -18.0,
        //         )),
        //         math::vector::Dir3::new(130.0, 0.0, 65.0),
        //     )),
        //     0.01,
        //     Arc::new(isotropic::Isotropic::new(solid_texture::SolidTexture::new(
        //         math::vector::Color3::one(),
        //     ))),
        // )));

        objects.push(Arc::new(translate::Translate::new(
            Arc::new(rotate::RotateY::new(
                Arc::new(cubic::Cubic::new(
                    math::vector::Point3::new(0.0, 0.0, 0.0),
                    math::vector::Point3::new(165.0, 330.0, 165.0),
                    white.clone(),
                )),
                15.0,
            )),
            math::vector::Dir3::new(265.0, 0.0, 295.0),
        )));
        objects.push(Arc::new(translate::Translate::new(
            Arc::new(rotate::RotateY::new(
                Arc::new(cubic::Cubic::new(
                    math::vector::Point3::new(0.0, 0.0, 0.0),
                    math::vector::Point3::new(165.0, 165.0, 165.0),
                    white.clone(),
                )),
                -18.0,
            )),
            math::vector::Dir3::new(130.0, 0.0, 65.0),
        )));

        let lights = Arc::new(rect::XZRect::new(
            213.0, 343.0, 227.0, 332.0, 554.0, difflight,
        ));
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
        (
            objects,
            lights,
            cam,
            math::vector::Color3::new(0.0, 0.0, 0.0),
        )
    }
    fn final_scene() -> (Vec<Arc<dyn Hittable>>, camera::Camera, vector::Color3) {
        let mut objects: Vec<Arc<dyn Hittable>> = vec![];
        // backgound
        let ground = Arc::new(material::lambertian::Lambertian::new(
            solid_texture::SolidTexture::new(math::vector::Color3::new(0.48, 0.83, 0.53)),
        ));
        let mut boxes1 = HittableList::new();
        const BOXES_PER_SIDE: usize = 20;
        for i in 0..BOXES_PER_SIDE {
            for j in 0..BOXES_PER_SIDE {
                let w = 100.0;
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = math::random::generate_range(1.0, 101.0);
                let z1 = z0 + w;

                boxes1.add(Arc::new(cubic::Cubic::new(
                    vector::Point3::new(x0, y0, z0),
                    vector::Point3::new(x1, y1, z1),
                    ground.clone(),
                )));
            }
        }
        objects.push(Arc::new(boxes1));

        // light
        let lightcolor = solid_texture::SolidTexture::new(math::vector::Color3::new(7.0, 7.0, 7.0));
        let difflight = Arc::new(diffuse_light::DiffuseLight::new(lightcolor));

        objects.push(Arc::new(rect::XZRect::new(
            123.0, 423.0, 147.0, 412.0, 554.0, difflight,
        )));

        // sphere
        let center1 = vector::Point3::new(400.0, 400.0, 200.0);
        let center2 = center1.clone() + vector::Dir3::new(30.0, 0.0, 0.0);
        let moving_sphere_material = Arc::new(material::lambertian::Lambertian::new(
            solid_texture::SolidTexture::new(math::vector::Color3::new(0.7, 0.3, 0.1)),
        ));
        objects.push(Arc::new(sphere::Sphere::new_move(
            center1,
            center2,
            50.0,
            moving_sphere_material,
            0.0,
            1.0,
        )));
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(260.0, 150.0, 45.0),
            50.0,
            Arc::new(material::dielectric::Dielectric::new(1.5)),
        )));
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 150.0, 145.0),
            50.0,
            Arc::new(material::metal::Metal::new(
                vector::Color3::new(0.8, 0.8, 0.9),
                1.0,
            )),
        )));
        let boundary = Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(360.0, 150.0, 145.0),
            70.0,
            Arc::new(material::dielectric::Dielectric::new(1.5)),
        ));

        objects.push(boundary.clone());
        objects.push(Arc::new(constant_medium::ConstantMedium::new(
            boundary,
            0.2,
            Arc::new(isotropic::Isotropic::new(solid_texture::SolidTexture::new(
                math::vector::Color3::new(0.2, 0.4, 0.9),
            ))),
        )));
        let boundary2 = Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 0.0, 0.0),
            5000.0,
            Arc::new(material::dielectric::Dielectric::new(1.5)),
        ));

        objects.push(Arc::new(constant_medium::ConstantMedium::new(
            boundary2,
            0.0001,
            Arc::new(isotropic::Isotropic::new(solid_texture::SolidTexture::new(
                math::vector::Color3::new(1.0, 1.0, 1.0),
            ))),
        )));

        let mut pic = picture::Jpeg::new("asset/earthmap.jpg".to_string());
        let mut bitmap = bitmap::Bitmap::new_empty();
        pic.load(&mut bitmap);

        let earth_surface = image_texture::ImageTexture::new(Arc::new(bitmap));
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(400.0, 200.0, 400.0),
            100.0,
            Arc::new(material::lambertian::Lambertian::new(earth_surface)),
        )));

        let perlintexure = noise_texture::NoiseTexture::new(0.1);
        objects.push(Arc::new(sphere::Sphere::new(
            math::vector::Point3::new(220.0, 280.0, 300.0),
            80.0,
            Arc::new(material::lambertian::Lambertian::new(perlintexure)),
        )));

        // Box
        let mut boxes2 = HittableList::new();
        let write = Arc::new(lambertian::Lambertian::new(
            solid_texture::SolidTexture::new(vector::Color3::new(0.73, 0.73, 0.73)),
        ));
        const NS: usize = 1000;
        for _ in 0..NS {
            boxes2.add(Arc::new(sphere::Sphere::new(
                vector::Point3::random_range(0.0, 165.0),
                10.0,
                write.clone(),
            )));
        }
        objects.push(Arc::new(translate::Translate::new(
            Arc::new(rotate::RotateY::new(Arc::new(boxes2), 15.0)),
            math::vector::Dir3::new(-100.0, 270.0, 395.0),
        )));

        // camera
        let look_from = math::vector::Point3::new(478.0, 278.0, -600.0);
        let look_at = math::vector::Point3::new(278.0, 278.0, 0.0);
        let focus_dist = 50.0;
        let cam = camera::Camera::new(
            look_from,
            look_at,
            math::vector::Dir3::new(0.0, 1.0, 0.0),
            40.0,
            0.1,
            focus_dist,
            0.0,
            1.0,
        );
        let backgound = math::vector::Color3::new(0.0, 0.0, 0.0);
        (objects, cam, backgound)
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
    pub fn lights(&self) -> Arc<dyn Hittable> {
        self.lights.clone()
    }
}
