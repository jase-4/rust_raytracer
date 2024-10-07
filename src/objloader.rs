use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct OBJLoader {
    pub vertices: Vec<(f64, f64, f64)>,
    pub normals: Vec<(f64, f64, f64)>,
    pub faces: Vec<Vec<(usize, usize)>>,
}

impl OBJLoader {
    pub fn new() -> Self {
        OBJLoader {
            vertices: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn load_obj(&mut self, filename: &str) -> io::Result<()> {
        let path = Path::new(filename);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.starts_with("v ") {
                self.parse_vertex(&line);
            } else if line.starts_with("vn ") {
                self.parse_normal(&line);
            } else if line.starts_with("f ") {
                self.parse_face(&line);
            }
        }

        Ok(())
    }

    fn parse_vertex(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 4 {
            let x = parts[1].parse::<f64>().unwrap();
            let y = parts[2].parse::<f64>().unwrap();
            let z = parts[3].parse::<f64>().unwrap();
            self.vertices.push((x, y, z));
        }
    }

    fn parse_normal(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 4 {
            let nx = parts[1].parse::<f64>().unwrap();
            let ny = parts[2].parse::<f64>().unwrap();
            let nz = parts[3].parse::<f64>().unwrap();
            self.normals.push((nx, ny, nz));
        }
    }

    fn parse_face(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut face = Vec::new();
        for part in &parts[1..] {
            let indices: Vec<&str> = part.split("//").collect();
            let vertex_index = indices[0].parse::<usize>().unwrap();
            let normal_index = indices[1].parse::<usize>().unwrap();
            face.push((vertex_index, normal_index));
        }
        self.faces.push(face);
    }

    fn display_data(&self) {
        println!("Vertices:");
        for v in &self.vertices {
            println!("{:?}", v);
        }

        println!("\nNormals:");
        for n in &self.normals {
            println!("{:?}", n);
        }

        println!("\nFaces:");
        for f in &self.faces {
            println!("{:?}", f);
        }
    }
}
