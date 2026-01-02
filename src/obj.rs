use crate::objloader::OBJLoader;
use std::rc::Rc;
use std::sync::Arc;

use crate::triangle;
use crate::vec3::Vec3;
use crate::{material::Material, triangle::Triangle, vec3::Point3};

pub fn load_obj_file(file_path: &str, material: Arc<dyn Material>) -> Vec<Triangle> {
    let mut triangles: Vec<Triangle> = Vec::new();

    let mut loader = OBJLoader::new();
    loader.load_obj(file_path).unwrap();
    println!("gaces {:?}", loader.faces[0][0].0);
    println!("gaces {:?}", loader.faces[0][1].0);
    println!("gaces {:?}", loader.faces[0][2].0);
    println!("gaces {:?}", loader.vertices[loader.faces[0][0].0]);
    println!("gaces {:?}", loader.vertices[loader.faces[0][1].0]);
    println!("gaces {:?}", loader.vertices[loader.faces[0][2].0]);
    println!("len verts {:?}", loader.vertices.len());
    println!("len faces {:?}", loader.faces.len());

    for face in loader.faces {
        let f0 = face[0].0;
        let f1 = face[1].0;
        let f2 = face[2].0;
        println!("{}", f0);
        println!("{}", f1);
        println!("{}", f2);
        let triangle = Triangle::new(
            Vec3::new(
                loader.vertices[f0 - 1].0,
                loader.vertices[f0 - 1].1,
                loader.vertices[f0 - 1].2,
            ),
            Vec3::new(
                loader.vertices[f1 - 1].0,
                loader.vertices[f1 - 1].1,
                loader.vertices[f1 - 1].2,
            ),
            Vec3::new(
                loader.vertices[f2 - 1].0,
                loader.vertices[f2 - 1].1,
                loader.vertices[f2 - 1].2,
            ),
            Arc::clone(&material),
        );

        triangles.push(triangle);
    }

    triangles
}
