use image::{ImageBuffer, Rgb};
use m1_raytracer::{
    camera::{self, Camera, CameraParams},
    color::Color,
    hittable_list::{self, HittableList},
    material::Lambertian,
    sphere::{self, Sphere},
    triangle::{self, create_cube, Triangle},
    vec3::{Point3, Vec3},
};
use std::rc::Rc;

use m1_raytracer::obj::load_obj_file;

fn main() {
    let mut world = HittableList::new();
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let sphere = Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, -50.0),
        1000.0,
        ground_material,
    ));
    world.add(sphere);

    let cube_mat = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
    //let ground_material1 = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));

    
    let cube = create_cube(cube_mat, 2.0);
    for triangle in cube {
            world.add(Rc::new(triangle));
    }
    

    // let list_triangles = load_obj_file("armadillo_lowres.obj", ground_material);
    // for triangle in list_triangles {
    //     world.add(Rc::new(triangle));
    // }

    

    let sphere_mat = Rc::new(Lambertian::new(Color::new(0.0, 1.0, 0.0)));
    let sphere2 = Rc::new(Sphere::new(Point3::new(5.0, 3.0, 0.0), 0.5, sphere_mat));

    let sphere_mat2 = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let sphere3 = Rc::new(Sphere::new(Point3::new(0.0, 1.0, -5.0), 1.0, sphere_mat2));

     world.add(sphere2);
     world.add(sphere3);

    let mut cam = Camera::new_default();
    //let fov_y = 45.0f64.to_radians(); // Convert degrees to radians

    cam.set_basic_params(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        img_width: 1200,
        samples_per_pixel: 10,
        max_depth: 5,
        vfov: 90.0,
        lookfrom: Point3::new(0.0, 1.0, 10.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
    });

    cam.render(&world);
}
