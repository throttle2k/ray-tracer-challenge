use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use crate::{
    shapes::{ObjectBuilder, WithGroup, WithShape},
    tuples::{points::Point, vectors::Vector, Tuple},
};
use anyhow::{anyhow, Result};

enum OBJElement {
    Vertex(Point),
    Face(Vec<FaceInfo>),
    Group(String),
    Normal(Vector),
}

#[derive(Debug, Clone, Copy)]
pub struct FaceInfo {
    vertex_index: usize,
    texture_vertex_index: Option<usize>,
    vertex_normal_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    group: Option<String>,
    vertices: Vec<Point>,
    normals: Vec<Vector>,
    textures: Vec<usize>,
}

impl Face {
    fn new() -> Self {
        Self {
            group: None,
            vertices: Vec::new(),
            normals: Vec::new(),
            textures: Vec::new(),
        }
    }

    fn get_vertex(&self, idx: usize) -> Result<Point> {
        if idx > self.vertices.len() {
            Err(anyhow!(OBJParserError::ObjectNotFound(
                "Vertex".into(),
                idx
            )))
        } else {
            Ok(self.vertices[idx - 1])
        }
    }

    fn get_normal(&self, idx: usize) -> Result<Vector> {
        if idx > self.vertices.len() {
            Err(anyhow!(OBJParserError::ObjectNotFound(
                "Normal".into(),
                idx
            )))
        } else {
            Ok(self.normals[idx - 1])
        }
    }

    fn push_vertex(&mut self, v: Point) {
        self.vertices.push(v)
    }

    fn push_normal(&mut self, n: Vector) {
        self.normals.push(n)
    }

    fn push_texture(&mut self, t: usize) {
        self.textures.push(t)
    }
}

pub struct OBJParser {
    lines_skipped: usize,
    vertices: Vec<Point>,
    faces: Vec<Face>,
    normals: Vec<Vector>,
}

impl OBJParser {
    pub fn load_file(path: &PathBuf) -> Result<Self> {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        Ok(Self::parse(&input))
    }

    pub fn parse(input: &str) -> Self {
        let mut lines_skipped = 0;
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let mut current_group = None;
        let mut normals = Vec::new();
        for line in input.lines() {
            let line_vec: Vec<&str> = line.split_whitespace().collect();
            match parse_line(&line_vec) {
                Ok(OBJElement::Vertex(p)) => vertices.push(p),
                Ok(OBJElement::Face(fv)) => {
                    if let Some((p1, rest)) = fv.split_first() {
                        for window in rest.windows(2) {
                            let p2 = window[0];
                            let p3 = window[1];
                            let mut f = Face::new();
                            f.group = current_group.clone();
                            f.push_vertex(vertices[p1.vertex_index - 1]);
                            f.push_vertex(vertices[p2.vertex_index - 1]);
                            f.push_vertex(vertices[p3.vertex_index - 1]);
                            if let Some(v1) = p1.vertex_normal_index {
                                f.push_normal(normals[v1 - 1]);
                            }
                            if let Some(v2) = p2.vertex_normal_index {
                                f.push_normal(normals[v2 - 1]);
                            }
                            if let Some(v3) = p3.vertex_normal_index {
                                f.push_normal(normals[v3 - 1]);
                            }
                            if let Some(t1) = p1.texture_vertex_index {
                                f.push_texture(t1);
                            }
                            if let Some(t2) = p2.texture_vertex_index {
                                f.push_texture(t2);
                            }
                            if let Some(t3) = p3.texture_vertex_index {
                                f.push_texture(t3);
                            }
                            faces.push(f);
                        }
                    }
                }
                Ok(OBJElement::Group(name)) => current_group = Some(name),
                Ok(OBJElement::Normal(vn)) => normals.push(vn),
                Err(_) => lines_skipped += 1,
            }
        }
        Self {
            lines_skipped,
            vertices,
            faces,
            normals,
        }
    }

    pub fn get_vertex(&self, idx: usize) -> Result<Point> {
        if idx > self.vertices.len() {
            Err(anyhow!(OBJParserError::ObjectNotFound(
                "Vertex".into(),
                idx
            )))
        } else {
            Ok(self.vertices[idx - 1])
        }
    }

    pub fn get_face(&self, idx: usize) -> Result<Face> {
        if idx > self.faces.len() {
            Err(anyhow!(OBJParserError::ObjectNotFound("Face".into(), idx)))
        } else {
            Ok(self.faces[idx - 1].clone())
        }
    }

    pub fn get_normal(&self, idx: usize) -> Result<Vector> {
        if idx > self.normals.len() {
            Err(anyhow!(OBJParserError::ObjectNotFound(
                "Normal".into(),
                idx
            )))
        } else {
            Ok(self.normals[idx - 1].clone())
        }
    }

    pub fn into_group(&self) -> ObjectBuilder<WithShape, WithGroup> {
        let mut groups: BTreeMap<Option<String>, Vec<Face>> = BTreeMap::new();
        for face in self.faces.iter() {
            groups
                .entry(face.group.clone())
                .and_modify(|e| e.push(face.clone()))
                .or_insert(vec![face.clone()]);
        }
        let mut g = ObjectBuilder::new_group();
        if let Some(default_group) = groups.get(&None) {
            for face in default_group {
                let t = if face.normals.is_empty() {
                    ObjectBuilder::new_triangle()
                        .set_p1(face.get_vertex(1).unwrap())
                        .set_p2(face.get_vertex(2).unwrap())
                        .set_p3(face.get_vertex(3).unwrap())
                        .build()
                } else {
                    ObjectBuilder::new_smooth_triangle()
                        .set_p1(face.get_vertex(1).unwrap())
                        .set_p2(face.get_vertex(2).unwrap())
                        .set_p3(face.get_vertex(3).unwrap())
                        .set_n1(face.get_normal(1).unwrap())
                        .set_n2(face.get_normal(2).unwrap())
                        .set_n3(face.get_normal(3).unwrap())
                        .build()
                };
                g = g.add_child(t);
            }
        }
        for (_, faces) in groups.iter().filter(|&(k, _)| *k != None) {
            let mut new_group = ObjectBuilder::new_group();
            for face in faces {
                let t = if face.normals.is_empty() {
                    ObjectBuilder::new_triangle()
                        .set_p1(face.get_vertex(1).unwrap())
                        .set_p2(face.get_vertex(2).unwrap())
                        .set_p3(face.get_vertex(3).unwrap())
                        .build()
                } else {
                    ObjectBuilder::new_smooth_triangle()
                        .set_p1(face.get_vertex(1).unwrap())
                        .set_p2(face.get_vertex(2).unwrap())
                        .set_p3(face.get_vertex(3).unwrap())
                        .set_n1(face.get_normal(1).unwrap())
                        .set_n2(face.get_normal(2).unwrap())
                        .set_n3(face.get_normal(3).unwrap())
                        .build()
                };
                new_group = new_group.add_child(t);
            }
            g = g.add_child(new_group.build());
        }
        g
    }
}

fn parse_line(line: &[&str]) -> Result<OBJElement> {
    if line.is_empty() {
        return Err(anyhow!(OBJParserError::ParseError(
            "Line".into(),
            "Empty line".into()
        )));
    }
    let e = match line[0] {
        "v" => parse_vertex(&line[1..]),
        "f" => parse_face(&line[1..]),
        "g" => parse_group(&line[1..]),
        "vn" => parse_vertex_normal(&line[1..]),
        _ => Err(anyhow!("Unrecognized element: {}", line[0])),
    };
    e
}

fn parse_vertex(line: &[&str]) -> Result<OBJElement> {
    if line.len() != 3 {
        return Err(anyhow!(OBJParserError::ParseError(
            "Vertex".into(),
            "Wrong format".into()
        )));
    }
    let x = line[0].parse::<f64>()?;
    let y = line[1].parse::<f64>()?;
    let z = line[2].parse::<f64>()?;
    Ok(OBJElement::Vertex(Point::new(x, y, z)))
}

fn parse_face(line: &[&str]) -> Result<OBJElement> {
    let vertices: Vec<FaceInfo> = line
        .iter()
        .map(|s| {
            let mut splits = s.split("/");
            let v = splits.next().unwrap();
            let v = v.parse::<usize>();
            let tv = if let Some(tv) = splits.next() {
                if !tv.is_empty() {
                    Some(tv.parse::<usize>().unwrap())
                } else {
                    None
                }
            } else {
                None
            };
            let vn = if let Some(vn) = splits.next() {
                if !vn.is_empty() {
                    Some(vn.parse::<usize>().unwrap())
                } else {
                    None
                }
            } else {
                None
            };
            FaceInfo {
                vertex_index: v.unwrap(),
                texture_vertex_index: tv,
                vertex_normal_index: vn,
            }
        })
        .collect();
    Ok(OBJElement::Face(vertices))
}

fn parse_group(line: &[&str]) -> Result<OBJElement> {
    if line.is_empty() {
        return Err(anyhow!(OBJParserError::ParseError(
            "Group".into(),
            "Missing name".into()
        )));
    }
    let s = line.join(" ");
    Ok(OBJElement::Group(s))
}

fn parse_vertex_normal(line: &[&str]) -> Result<OBJElement> {
    if line.len() != 3 {
        return Err(anyhow!(OBJParserError::ParseError(
            "Vertex".into(),
            "Wrong format".into()
        )));
    }
    let x = line[0].parse::<f64>()?;
    let y = line[1].parse::<f64>()?;
    let z = line[2].parse::<f64>()?;
    Ok(OBJElement::Normal(Vector::new(x, y, z)))
}

#[derive(Debug)]
enum OBJParserError {
    ObjectNotFound(String, usize),
    ParseError(String, String),
}

impl std::fmt::Display for OBJParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OBJParserError::ObjectNotFound(obj_type, idx) => {
                write!(f, "Could not find {obj_type} with index {idx}")
            }
            OBJParserError::ParseError(obj_type, error) => {
                write!(f, "Could not parse {obj_type}: {error}")
            }
        }
    }
}

impl std::error::Error for OBJParserError {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ignoring_unrecognized_lines() {
        let input = r#"There was a young lady named Bright
            who traveled much faster than light.
            She set out one day
            in a relative way,
            and came back the previous night."#;
        let parser = OBJParser::parse(input);
        assert_eq!(parser.lines_skipped, 5);
    }

    #[test]
    fn vertex_records() {
        let input = r#"v -1 1 0
            v -1.0000 0.5000 0.0000
            v 1 0 0
            v 1 1 0"#;
        let parser = OBJParser::parse(input);
        assert_eq!(parser.get_vertex(1).unwrap(), Point::new(-1.0, 1.0, 0.0));
        assert_eq!(parser.get_vertex(2).unwrap(), Point::new(-1.0, 0.5, 0.0));
        assert_eq!(parser.get_vertex(3).unwrap(), Point::new(1.0, 0.0, 0.0));
        assert_eq!(parser.get_vertex(4).unwrap(), Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn parsing_triangle_faces() {
        let input = r#"v -1 1 0
            v -1 0 0
            v 1 0 0
            v 1 1 0
            f 1 2 3
            f 1 3 4"#;
        let parser = OBJParser::parse(input);
        assert!(parser.get_face(1).unwrap().group.is_none());
        assert!(parser.get_face(2).unwrap().group.is_none());
        assert_eq!(
            parser.get_face(1).unwrap().get_vertex(1).unwrap(),
            parser.get_vertex(1).unwrap()
        );
        assert_eq!(
            parser.get_face(1).unwrap().get_vertex(2).unwrap(),
            parser.get_vertex(2).unwrap()
        );
        assert_eq!(
            parser.get_face(1).unwrap().get_vertex(3).unwrap(),
            parser.get_vertex(3).unwrap()
        );
        assert_eq!(
            parser.get_face(2).unwrap().get_vertex(1).unwrap(),
            parser.get_vertex(1).unwrap()
        );
        assert_eq!(
            parser.get_face(2).unwrap().get_vertex(2).unwrap(),
            parser.get_vertex(3).unwrap()
        );
        assert_eq!(
            parser.get_face(2).unwrap().get_vertex(3).unwrap(),
            parser.get_vertex(4).unwrap()
        );
    }

    #[test]
    fn triangulating_polygons() {
        let input = r#"v -1 1 0
            v -1 0 0
            v 1 0 0
            v 1 1 0
            v 0 2 0
            f 1 2 3 4 5"#;
        let parser = OBJParser::parse(input);
        assert!(parser.get_face(1).unwrap().group.is_none());
        assert!(parser.get_face(2).unwrap().group.is_none());
        assert!(parser.get_face(3).unwrap().group.is_none());
        assert_eq!(
            parser.get_face(1).unwrap().get_vertex(1).unwrap(),
            parser.get_vertex(1).unwrap()
        );
        assert_eq!(
            parser.get_face(1).unwrap().get_vertex(2).unwrap(),
            parser.get_vertex(2).unwrap()
        );
        assert_eq!(
            parser.get_face(1).unwrap().get_vertex(3).unwrap(),
            parser.get_vertex(3).unwrap()
        );
        assert_eq!(
            parser.get_face(2).unwrap().get_vertex(1).unwrap(),
            parser.get_vertex(1).unwrap()
        );
        assert_eq!(
            parser.get_face(2).unwrap().get_vertex(2).unwrap(),
            parser.get_vertex(3).unwrap()
        );
        assert_eq!(
            parser.get_face(2).unwrap().get_vertex(3).unwrap(),
            parser.get_vertex(4).unwrap()
        );
        assert_eq!(
            parser.get_face(3).unwrap().get_vertex(1).unwrap(),
            parser.get_vertex(1).unwrap()
        );
        assert_eq!(
            parser.get_face(3).unwrap().get_vertex(2).unwrap(),
            parser.get_vertex(4).unwrap()
        );
        assert_eq!(
            parser.get_face(3).unwrap().get_vertex(3).unwrap(),
            parser.get_vertex(5).unwrap()
        );
    }

    #[test]
    fn triangles_in_groups() {
        let input = r#"v -1 1 0
            v -1 0 0
            v 1 0 0
            v 1 1 0
            g FirstGroup
            f 1 2 3
            g SecondGroup
            f 1 3 4"#;
        let parser = OBJParser::parse(input);
        assert_eq!(parser.get_face(1).unwrap().group, Some("FirstGroup".into()));
        assert_eq!(
            parser.get_face(2).unwrap().group,
            Some("SecondGroup".into())
        );
        assert_eq!(
            parser.get_face(1).unwrap().get_vertex(1).unwrap(),
            parser.get_vertex(1).unwrap()
        );
        assert_eq!(
            parser.get_face(1).unwrap().get_vertex(2).unwrap(),
            parser.get_vertex(2).unwrap()
        );
        assert_eq!(
            parser.get_face(1).unwrap().get_vertex(3).unwrap(),
            parser.get_vertex(3).unwrap()
        );
        assert_eq!(
            parser.get_face(2).unwrap().get_vertex(1).unwrap(),
            parser.get_vertex(1).unwrap()
        );
        assert_eq!(
            parser.get_face(2).unwrap().get_vertex(2).unwrap(),
            parser.get_vertex(3).unwrap()
        );
        assert_eq!(
            parser.get_face(2).unwrap().get_vertex(3).unwrap(),
            parser.get_vertex(4).unwrap()
        );
    }

    #[test]
    fn converting_an_obj_file_to_group() {
        let input = r#"v -1 1 0
            v -1 0 0
            v 1 0 0
            v 1 1 0
            g FirstGroup
            f 1 2 3
            g SecondGroup
            f 1 3 4"#;
        let parser = OBJParser::parse(input);
        let g = parser.into_group().build();
        let c = g.group().unwrap().children();
        assert_eq!(c.len(), 2);
        let g1 = &c[0];
        let g2 = &c[1];
        let c1 = g1.group().unwrap().children();
        let c2 = g2.group().unwrap().children();
        assert_eq!(c1.len(), 1);
        assert_eq!(c2.len(), 1);
        let v1 = &c1[0];
        let v2 = &c2[0];
        assert_eq!(v1.p1().unwrap(), Point::new(-1.0, 1.0, 0.0));
        assert_eq!(v1.p2().unwrap(), Point::new(-1.0, 0.0, 0.0));
        assert_eq!(v1.p3().unwrap(), Point::new(1.0, 0.0, 0.0));
        assert_eq!(v2.p1().unwrap(), Point::new(-1.0, 1.0, 0.0));
        assert_eq!(v2.p2().unwrap(), Point::new(1.0, 0.0, 0.0));
        assert_eq!(v2.p3().unwrap(), Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn parsing_vertex_normals() {
        let input = r#"vn 0 0 1
            vn 0.707 0 -0.707
            vn 1 2 3"#;
        let parser = OBJParser::parse(input);
        assert_eq!(parser.get_normal(1).unwrap(), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(
            parser.get_normal(2).unwrap(),
            Vector::new(0.707, 0.0, -0.707)
        );
        assert_eq!(parser.get_normal(3).unwrap(), Vector::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn faces_with_normals() {
        let input = r#"v 0 1 0
            v -1 0 0
            v 1 0 0
            vn -1 0 0
            vn 1 0 0
            vn 0 1 0
            f 1//3 2//1 3//2
            f 1/0/3 2/102/1 3/14/2"#;
        let parser = OBJParser::parse(input);
        let g = parser.into_group().build();
        let c = g.group().unwrap().children();
        assert_eq!(c.len(), 2);
        let t1 = &c[0];
        let t2 = &c[1];
        assert_eq!(t1.p1().unwrap(), parser.get_vertex(1).unwrap());
        assert_eq!(t1.p2().unwrap(), parser.get_vertex(2).unwrap());
        assert_eq!(t1.p3().unwrap(), parser.get_vertex(3).unwrap());
        assert_eq!(t1.n1().unwrap(), parser.get_normal(3).unwrap());
        assert_eq!(t1.n2().unwrap(), parser.get_normal(1).unwrap());
        assert_eq!(t1.n3().unwrap(), parser.get_normal(2).unwrap());
        assert_eq!(t2.p1().unwrap(), parser.get_vertex(1).unwrap());
        assert_eq!(t2.p2().unwrap(), parser.get_vertex(2).unwrap());
        assert_eq!(t2.p3().unwrap(), parser.get_vertex(3).unwrap());
        assert_eq!(t2.n1().unwrap(), parser.get_normal(3).unwrap());
        assert_eq!(t2.n2().unwrap(), parser.get_normal(1).unwrap());
        assert_eq!(t2.n3().unwrap(), parser.get_normal(2).unwrap());
    }
}
