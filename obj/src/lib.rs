type Point3 = (f64, f64, f64);

pub struct Obj<'a> {
    raw: &'a str,
    vertexes: Vec<Point3>,
    faces: Vec<[usize; 3]>,
}

impl<'a> Obj<'a> {
    pub fn new(raw: &'a str) -> Self {
        let mut vertexes = vec![];
        let mut faces = vec![];

        for line in raw.lines() {
            if line.starts_with("v ") {
                let mut coords = line.split_whitespace().skip(1);
                let x = coords.next().unwrap().parse::<f64>().unwrap();
                let y = coords.next().unwrap().parse::<f64>().unwrap();
                let z = coords.next().unwrap().parse::<f64>().unwrap();

                vertexes.push((x, y, z));
            }

            if line.starts_with("f ") {
                let mut coords = line.split_whitespace().skip(1);
                let point0 = coords.next().unwrap().parse::<usize>().unwrap();
                let point1 = coords.next().unwrap().parse::<usize>().unwrap();
                let point2 = coords.next().unwrap().parse::<usize>().unwrap();

                faces.push([point0, point1, point2]);
            }
        }

        Self {
            raw,
            vertexes,
            faces,
        }
    }

    pub fn vertexes(&self) -> &[Point3] {
        &self.vertexes
    }

    pub fn faces(&self) -> &[[usize; 3]] {
        &self.faces
    }

    pub fn face_iter(&self) -> impl Iterator<Item = [&Point3; 3]> {
        self.faces.iter().map(|face| {
            [
                &self.vertexes[face[0] - 1],
                &self.vertexes[face[1] - 1],
                &self.vertexes[face[2] - 1],
            ]
        })
    }
}
