use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use crate::{
    shapes::ObjectBuilder,
    tuples::{points::Point, Tuple},
};
use anyhow::{anyhow, Result};

enum OBJElement {
    Vertex(Point),
    Face(Vec<usize>),
    Group(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    group: Option<String>,
    vertices: Vec<Point>,
}

impl Face {
    fn new() -> Self {
        Self {
            group: None,
            vertices: Vec::new(),
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

    fn push(&mut self, v: Point) {
        self.vertices.push(v)
    }
}

pub struct OBJParser {
    lines_skipped: usize,
    vertices: Vec<Point>,
    faces: Vec<Face>,
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
                            f.push(vertices[p1 - 1]);
                            f.push(vertices[p2 - 1]);
                            f.push(vertices[p3 - 1]);
                            faces.push(f);
                        }
                    }
                }
                Ok(OBJElement::Group(name)) => current_group = Some(name),
                Err(_) => lines_skipped += 1,
            }
        }
        Self {
            lines_skipped,
            vertices,
            faces,
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

    pub fn into_group(&self) -> usize {
        let mut groups: HashMap<Option<String>, Vec<Face>> = HashMap::new();
        for face in self.faces.iter() {
            groups
                .entry(face.group.clone())
                .and_modify(|e| e.push(face.clone()))
                .or_insert(vec![face.clone()]);
        }
        let mut g = ObjectBuilder::new_group();
        if let Some(default_group) = groups.get(&None) {
            for face in default_group {
                let t = ObjectBuilder::new_triangle(
                    face.get_vertex(1).unwrap(),
                    face.get_vertex(2).unwrap(),
                    face.get_vertex(3).unwrap(),
                )
                .register();
                g = g.clone().add_child(t);
            }
        }
        for (_, faces) in groups.iter().filter(|&(k, _)| *k != None) {
            let mut new_group = ObjectBuilder::new_group();
            for face in faces {
                let t = ObjectBuilder::new_triangle(
                    face.get_vertex(1).unwrap(),
                    face.get_vertex(2).unwrap(),
                    face.get_vertex(3).unwrap(),
                )
                .register();
                new_group = new_group.clone().add_child(t);
            }
            g = g.clone().add_child(new_group.register());
        }
        g.register()
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
    let vertices: Result<Vec<_>, _> = line.iter().map(|s| s.parse::<usize>()).collect();
    Ok(OBJElement::Face(vertices?))
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
    use crate::REGISTRY;

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
        let g = parser.into_group();
        let registry = REGISTRY.read().unwrap();
        let g = registry.get_object(g).unwrap();
        let c = g.group().children();
        assert_eq!(c.len(), 2);
        let g1 = registry.get_object(c[0]).unwrap();
        let g2 = registry.get_object(c[1]).unwrap();
        let c1 = g1.group().children();
        let c2 = g2.group().children();
        assert_eq!(c1.len(), 1);
        assert_eq!(c2.len(), 1);
        let v1 = registry.get_object(c1[0]).unwrap();
        let v2 = registry.get_object(c2[0]).unwrap();
        assert_eq!(v1.p1(), Point::new(-1.0, 1.0, 0.0));
        assert_eq!(v1.p2(), Point::new(-1.0, 0.0, 0.0));
        assert_eq!(v1.p3(), Point::new(1.0, 0.0, 0.0));
        assert_eq!(v2.p1(), Point::new(-1.0, 1.0, 0.0));
        assert_eq!(v2.p2(), Point::new(1.0, 0.0, 0.0));
        assert_eq!(v2.p3(), Point::new(1.0, 1.0, 0.0));
    }
}
