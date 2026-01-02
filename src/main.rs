use m1_raytracer::{
    camera::{self, Camera, CameraParams},
    color::Color,
    hittable_list::{self, HittableList},
    material::Lambertian,
    sphere::{self, Sphere},
    vec3::{Point3, Vec3},
};
use std::sync::Arc;
use m1_raytracer::obj::load_obj_file;
use m1_raytracer::material::{Dielectric, Metal};

fn main() {
    let mut world = HittableList::new();
    let ground_material = Arc::new(Lambertian::new(Color::new(0.4, 0.4, 0.4)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // let armadillo_material =  Arc::new(Metal::new(
    //     Color::new(0.2, 1.0, 0.2), 
    //     0.0, 
    // ));

    // println!("Loading armadillo model...");
    // let armadillo_triangles = load_obj_file("armadillo_lowres.obj", armadillo_material);
    // println!("Loaded {} triangles for armadillo", armadillo_triangles.len());
    
    // for triangle in armadillo_triangles {
    //     world.add(Arc::new(triangle));
    // }

    let light_material = Arc::new(Lambertian::new(Color::new(1.0, 0.95, 0.9)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 6.0, -4.0),
        1.0,
        light_material,
    )));

    let glass1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(2.5, 1.0, -3.0),
        1.2,
        glass1.clone(),
    )));

    let glass2 = Arc::new(Dielectric::new(1.8)); 
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 1.0, 2.0),
        0.9,
        glass2.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 1.0, 2.0),
        -0.8,
        glass2,
    )));
    let blue_metal = Arc::new(Metal::new(
        Color::new(0.2, 0.5, 0.95), 
        0.0, 
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.5, 0.7, 0.6), 
        0.7,
        blue_metal,
    )));

   
    let gold_metal = Arc::new(Metal::new(
        Color::new(1.0, 0.8, 0.3),
        0.05,
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.6, 1.0), 
        0.6,
        gold_metal,
    )));

    let red_material = Arc::new(Lambertian::new(Color::new(0.95, 0.05, 0.15)));
    world.add(Arc::new(Sphere::new(
        Point3::new(3.5, 0.8, 2.0),
        0.8,
        red_material,
    )));

    let mut cam = Camera::new_default();

    cam.set_basic_params(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        img_width: 1200,
        samples_per_pixel: 100,
        max_depth: 30,
        vfov: 45.0, 
        lookfrom: Point3::new(0.0, 5.5, -1.0),
        lookat: Point3::new(0.5, 0.0, 0.5), 
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.3,
        focus_dist: 7.0,
    });

    println!("Rendering scene with {} objects...", world.objects.len());
    
    cam.render(&world);
}