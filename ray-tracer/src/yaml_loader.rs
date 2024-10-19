use std::{fs, path::Path};

use serde::{
    de::{self, IntoDeserializer},
    Deserialize, Deserializer,
};

use crate::{
    camera::Camera,
    lights::PointLight,
    materials::Material,
    patterns::Pattern,
    ppm::PPM,
    shapes::{CSGKind, Cap, Object, ObjectBuilder},
    transformations::Transformation,
    world::World,
};

trait IntoWithDefines<T> {
    fn into_with_defines(self, defines: &[Define]) -> T;
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SceneCommand {
    Add(Add),
    Define(Define),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Add {
    AddLight(YamlLight),
    AddObject(YamlObject),
    AddCamera(YamlCamera),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
struct Define {
    define: DefinitionId,
    extend: Option<DefinitionId>,
    transform: Option<Vec<YamlTransform>>,
    material: Option<YamlMaterial>,
}

impl Define {
    fn expand(&self, defines: &[Define]) -> Self {
        if let Some(ext_define) = &self.extend {
            let ext_define = defines
                .iter()
                .find(|def| def.define == *ext_define)
                .unwrap()
                .expand(defines);
            if let Some(mut ext_transform) = ext_define.transform {
                let mut transform = self.transform.clone().unwrap();
                transform.append(&mut ext_transform);
                Define {
                    define: self.define.clone(),
                    extend: None,
                    transform: Some(transform),
                    material: None,
                }
            } else if let Some(ext_material) = ext_define.material {
                Define {
                    define: self.define.clone(),
                    extend: None,
                    transform: None,
                    material: Some(ext_material.merge(self.material.clone().unwrap())),
                }
            } else {
                unreachable!()
            }
        } else {
            self.clone()
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "camera", tag = "add")]
struct YamlCamera {
    width: usize,
    height: usize,
    #[serde(rename = "field-of-view")]
    field_of_view: f64,
    from: [f64; 3],
    to: [f64; 3],
    up: [f64; 3],
}

impl Into<Camera> for YamlCamera {
    fn into(self) -> crate::camera::Camera {
        Camera::new(self.width, self.height, self.field_of_view).with_transform(
            Transformation::view_transform(self.from.into(), self.to.into(), self.up.into()),
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "add", rename_all = "kebab-case")]
enum YamlLight {
    #[serde(alias = "point-light")]
    YamlPointLight { at: [f64; 3], intensity: [f64; 3] },
}

impl Into<crate::lights::PointLight> for YamlLight {
    fn into(self) -> crate::lights::PointLight {
        match self {
            YamlLight::YamlPointLight { at, intensity } => crate::lights::PointLight {
                position: at.into(),
                intensity: intensity.into(),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "add", rename_all = "kebab-case")]
enum YamlObject {
    Test {
        material: Option<MaterialOrReference>,
        transform: Option<Vec<TransformOrReference>>,
    },
    Sphere {
        material: Option<MaterialOrReference>,
        transform: Option<Vec<TransformOrReference>>,
    },
    Cube {
        material: Option<MaterialOrReference>,
        transform: Option<Vec<TransformOrReference>>,
    },
    Cone {
        min: Option<f64>,
        max: Option<f64>,
        cap: Option<YamlCap>,
        material: Option<MaterialOrReference>,
        transform: Option<Vec<TransformOrReference>>,
    },
    Cylinder {
        min: Option<f64>,
        max: Option<f64>,
        cap: Option<YamlCap>,
        material: Option<MaterialOrReference>,
        transform: Option<Vec<TransformOrReference>>,
    },
    Plane {
        material: Option<MaterialOrReference>,
        transform: Option<Vec<TransformOrReference>>,
    },
    Triangle {
        p1: [f64; 3],
        p2: [f64; 3],
        p3: [f64; 3],
        material: Option<MaterialOrReference>,
        transform: Option<Vec<TransformOrReference>>,
    },
    SmoothTriangle {
        p1: [f64; 3],
        p2: [f64; 3],
        p3: [f64; 3],
        n1: [f64; 3],
        n2: [f64; 3],
        n3: [f64; 3],
        material: Option<MaterialOrReference>,
        transform: Option<Vec<TransformOrReference>>,
    },
    Group {
        children: Vec<YamlObject>,
        transform: Option<Vec<TransformOrReference>>,
    },
    #[serde(alias = "csg")]
    CSG {
        kind: YamlCSGKind,
        left: Box<YamlObject>,
        right: Box<YamlObject>,
        transform: Option<Vec<TransformOrReference>>,
    },
}

impl IntoWithDefines<Object> for YamlObject {
    fn into_with_defines(self, defines: &[Define]) -> Object {
        match self {
            YamlObject::Test {
                material,
                transform,
            } => {
                let mut builder = ObjectBuilder::new_test_shape();
                if let Some(material) = material {
                    builder = builder.with_material(material.into_with_defines(defines));
                };
                if let Some(transform) = transform {
                    builder = builder.with_transform(transform.into_with_defines(defines));
                };
                builder.build()
            }
            YamlObject::Sphere {
                material,
                transform,
            } => {
                let mut builder = ObjectBuilder::new_sphere();
                if let Some(material) = material {
                    builder = builder.with_material(material.into_with_defines(defines));
                };
                if let Some(transform) = transform {
                    builder = builder.with_transform(transform.into_with_defines(defines));
                };
                builder.build()
            }
            YamlObject::Cube {
                material,
                transform,
            } => {
                let mut builder = ObjectBuilder::new_cube();
                if let Some(material) = material {
                    builder = builder.with_material(material.into_with_defines(defines));
                };
                if let Some(transform) = transform {
                    builder = builder.with_transform(transform.into_with_defines(defines));
                };
                builder.build()
            }
            YamlObject::Cone {
                min,
                max,
                cap,
                material,
                transform,
            } => {
                let mut builder = ObjectBuilder::new_cone();
                if let Some(min) = min {
                    builder = builder.with_min(min);
                };
                if let Some(max) = max {
                    builder = builder.with_max(max);
                };
                if let Some(cap) = cap {
                    builder = builder.with_cap(cap.into());
                };
                if let Some(material) = material {
                    builder = builder.with_material(material.into_with_defines(defines));
                };
                if let Some(transform) = transform {
                    builder = builder.with_transform(transform.into_with_defines(defines));
                };
                builder.build()
            }
            YamlObject::Cylinder {
                min,
                max,
                cap,
                material,
                transform,
            } => {
                let mut builder = ObjectBuilder::new_cylinder();
                if let Some(min) = min {
                    builder = builder.with_min(min);
                };
                if let Some(max) = max {
                    builder = builder.with_max(max);
                };
                if let Some(cap) = cap {
                    builder = builder.with_cap(cap.into());
                };
                if let Some(material) = material {
                    builder = builder.with_material(material.into_with_defines(defines));
                };
                if let Some(transform) = transform {
                    builder = builder.with_transform(transform.into_with_defines(defines));
                };
                builder.build()
            }
            YamlObject::Plane {
                material,
                transform,
            } => {
                let mut builder = ObjectBuilder::new_plane();
                if let Some(material) = material {
                    builder = builder.with_material(material.into_with_defines(defines));
                };
                if let Some(transform) = transform {
                    builder = builder.with_transform(transform.into_with_defines(defines));
                };
                builder.build()
            }
            YamlObject::Triangle {
                p1,
                p2,
                p3,
                material,
                transform,
            } => {
                let mut builder = ObjectBuilder::new_triangle()
                    .set_p1(p1.into())
                    .set_p2(p2.into())
                    .set_p3(p3.into());
                if let Some(material) = material {
                    builder = builder.with_material(material.into_with_defines(defines));
                };
                if let Some(transform) = transform {
                    builder = builder.with_transform(transform.into_with_defines(defines));
                };
                builder.build()
            }
            YamlObject::SmoothTriangle {
                p1,
                p2,
                p3,
                n1,
                n2,
                n3,
                material,
                transform,
            } => {
                let mut builder = ObjectBuilder::new_smooth_triangle()
                    .set_p1(p1.into())
                    .set_p2(p2.into())
                    .set_p3(p3.into())
                    .set_n1(n1.into())
                    .set_n2(n2.into())
                    .set_n3(n3.into());
                if let Some(material) = material {
                    builder = builder.with_material(material.into_with_defines(defines));
                };
                if let Some(transform) = transform {
                    builder = builder.with_transform(transform.into_with_defines(defines));
                };
                builder.build()
            }
            YamlObject::Group {
                children,
                transform,
            } => {
                let mut builder = children
                    .into_iter()
                    .fold(ObjectBuilder::new_group(), |b, child| {
                        b.add_child(child.into_with_defines(defines))
                    });
                if let Some(transform) = transform {
                    builder = builder.with_transform(transform.into_with_defines(defines));
                };
                builder.build()
            }
            YamlObject::CSG {
                kind,
                left,
                right,
                transform,
            } => {
                let mut builder = ObjectBuilder::new_csg(
                    kind.into(),
                    (*left).into_with_defines(defines),
                    (*right).into_with_defines(defines),
                );
                if let Some(transform) = transform {
                    builder = builder.with_transform(transform.into_with_defines(defines));
                };
                builder.build()
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum YamlCap {
    Uncapped,
    TopCap,
    BottomCap,
    Both,
}

impl Into<Cap> for YamlCap {
    fn into(self) -> Cap {
        match self {
            YamlCap::Uncapped => Cap::Uncapped,
            YamlCap::TopCap => Cap::TopCap,
            YamlCap::BottomCap => Cap::BottomCap,
            YamlCap::Both => Cap::Both,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum YamlCSGKind {
    Union,
    Intersection,
    Difference,
}

impl Into<CSGKind> for YamlCSGKind {
    fn into(self) -> CSGKind {
        match self {
            YamlCSGKind::Union => CSGKind::Union,
            YamlCSGKind::Intersection => CSGKind::Intersection,
            YamlCSGKind::Difference => CSGKind::Difference,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TransformOrReference {
    Transform(YamlTransform),
    Reference(DefinitionId),
}

impl<'de> Deserialize<'de> for TransformOrReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_yml::Value;

        let expr = Value::deserialize(deserializer)?;
        match expr {
            Value::String(s) => Ok(TransformOrReference::Reference(s)),
            Value::Sequence(seq) => {
                let transform = YamlTransform::deserialize(seq.into_deserializer()).unwrap();
                Ok(TransformOrReference::Transform(transform))
            }
            _ => Err(de::Error::custom(format!(
                "Invalid value for transform : {:?}",
                expr
            ))),
        }
    }
}

impl IntoWithDefines<Transformation> for TransformOrReference {
    fn into_with_defines(self, defines: &[Define]) -> Transformation {
        match self {
            TransformOrReference::Transform(t) => t.into(),
            TransformOrReference::Reference(r) => {
                let define = defines.iter().find(|def| def.define == r).unwrap();
                let transforms = define.transform.clone().unwrap();
                transforms.into()
            }
        }
    }
}

impl IntoWithDefines<Transformation> for Vec<TransformOrReference> {
    fn into_with_defines(self, defines: &[Define]) -> Transformation {
        let transformations: Vec<Transformation> = self
            .into_iter()
            .map(|tor| tor.into_with_defines(defines))
            .collect();
        let transformation = transformations
            .iter()
            .fold(Transformation::new_transform(), |t, m| {
                Transformation::from(&m.matrix * &t.matrix)
            });
        transformation
    }
}

impl Into<Transformation> for Vec<YamlTransform> {
    fn into(self) -> Transformation {
        let transformations: Vec<Transformation> = self.into_iter().map(Into::into).collect();
        let transformation = transformations
            .iter()
            .fold(Transformation::new_transform(), |t, m| {
                Transformation::from(&m.matrix * &t.matrix)
            });
        transformation
    }
}

#[derive(Debug, Clone, PartialEq)]
enum YamlTransform {
    Translate {
        x: f64,
        y: f64,
        z: f64,
    },
    Scale {
        x: f64,
        y: f64,
        z: f64,
    },
    RotateX {
        angle: f64,
    },
    RotateY {
        angle: f64,
    },
    RotateZ {
        angle: f64,
    },
    Shear {
        xy: f64,
        xz: f64,
        yx: f64,
        yz: f64,
        zx: f64,
        zy: f64,
    },
}

impl<'de> Deserialize<'de> for YamlTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_yml::Value;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        enum Op {
            Translate,
            Scale,
            RotateX,
            RotateY,
            RotateZ,
            Shear,
        }

        let expr = Vec::<Value>::deserialize(deserializer)?;
        let op = expr
            .get(0)
            .ok_or_else(|| de::Error::custom("Missing transform operator"))
            .and_then(|op| match op {
                Value::String(op) => serde_yml::from_str::<Op>(op),
                _ => Err(de::Error::custom(format!(
                    "Invalid transform operator: {op:?}"
                ))),
            })
            .map_err(de::Error::custom)?;
        let operands = &expr[1..];
        let transform = match op {
            Op::Translate => match operands {
                [Value::Number(x), Value::Number(y), Value::Number(z)] => {
                    YamlTransform::Translate {
                        x: x.as_f64().unwrap(),
                        y: y.as_f64().unwrap(),
                        z: z.as_f64().unwrap(),
                    }
                }
                _ => {
                    return Err(de::Error::custom(format!(
                        "Invalid transform translate operands: {operands:?}",
                    )))
                }
            },

            Op::Scale => match operands {
                [Value::Number(x), Value::Number(y), Value::Number(z)] => YamlTransform::Scale {
                    x: x.as_f64().unwrap(),
                    y: y.as_f64().unwrap(),
                    z: z.as_f64().unwrap(),
                },
                _ => {
                    return Err(de::Error::custom(format!(
                        "Invalid transform scale operands: {operands:?}",
                    )))
                }
            },

            Op::RotateX => match operands {
                [Value::Number(angle)] => YamlTransform::RotateX {
                    angle: angle.as_f64().unwrap(),
                },
                _ => {
                    return Err(de::Error::custom(format!(
                        "Invalid transform rotate-x operand value: {operands:?}",
                    )))
                }
            },

            Op::RotateY => match operands {
                [Value::Number(angle)] => YamlTransform::RotateY {
                    angle: angle.as_f64().unwrap(),
                },
                _ => {
                    return Err(de::Error::custom(format!(
                        "Invalid transform rotate-y operand: {operands:?}",
                    )))
                }
            },

            Op::RotateZ => match operands {
                [Value::Number(angle)] => YamlTransform::RotateZ {
                    angle: angle.as_f64().unwrap(),
                },
                _ => {
                    return Err(de::Error::custom(format!(
                        "Invalid transform rotate-z operand: {operands:?}",
                    )))
                }
            },

            Op::Shear => match operands {
                [Value::Number(xy), Value::Number(xz), Value::Number(yx), Value::Number(yz), Value::Number(zx), Value::Number(zy)] => {
                    YamlTransform::Shear {
                        xy: xy.as_f64().unwrap(),
                        xz: xz.as_f64().unwrap(),
                        yx: yx.as_f64().unwrap(),
                        yz: yz.as_f64().unwrap(),
                        zx: zx.as_f64().unwrap(),
                        zy: zy.as_f64().unwrap(),
                    }
                }
                _ => {
                    return Err(de::Error::custom(format!(
                        "Invalid transform shear operands: {operands:?}",
                    )))
                }
            },
        };

        Ok(transform)
    }
}

impl Into<Transformation> for YamlTransform {
    fn into(self) -> Transformation {
        match self {
            YamlTransform::Translate { x, y, z } => {
                Transformation::new_transform().translation(x, y, z)
            }
            YamlTransform::Scale { x, y, z } => Transformation::new_transform().scaling(x, y, z),
            YamlTransform::RotateX { angle } => Transformation::new_transform().rotation_x(angle),
            YamlTransform::RotateY { angle } => Transformation::new_transform().rotation_y(angle),
            YamlTransform::RotateZ { angle } => Transformation::new_transform().rotation_z(angle),
            YamlTransform::Shear {
                xy,
                xz,
                yx,
                yz,
                zx,
                zy,
            } => Transformation::new_transform().shearing(xy, xz, yx, yz, zx, zy),
        }
    }
}

#[derive(Debug)]
enum MaterialOrReference {
    Material(YamlMaterial),
    Reference(DefinitionId),
}

impl<'de> Deserialize<'de> for MaterialOrReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_yml::Value;

        let expr = Value::deserialize(deserializer)?;
        match expr {
            Value::String(s) => Ok(MaterialOrReference::Reference(s)),
            Value::Mapping(map) => {
                let material = serde_yml::from_value(Value::Mapping(map)).unwrap();
                Ok(MaterialOrReference::Material(material))
            }
            _ => Err(de::Error::custom(format!(
                "Invalid value for transform : {:?}",
                expr
            ))),
        }
    }
}

impl IntoWithDefines<Material> for MaterialOrReference {
    fn into_with_defines(self, defines: &[Define]) -> Material {
        match self {
            MaterialOrReference::Material(m) => m.into_with_defines(defines),
            MaterialOrReference::Reference(r) => {
                let define = defines.iter().find(|def| def.define == r).unwrap();
                let material = define.material.clone().unwrap();
                material.into_with_defines(defines)
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
struct YamlMaterial {
    pattern: Option<YamlPattern>,
    diffuse: Option<f64>,
    ambient: Option<f64>,
    specular: Option<f64>,
    shininess: Option<f64>,
    reflective: Option<f64>,
    transparency: Option<f64>,
    refractive_index: Option<f64>,
    cast_shadows: Option<bool>,
    receive_shadows: Option<bool>,
}

impl YamlMaterial {
    fn merge(&self, other: YamlMaterial) -> YamlMaterial {
        YamlMaterial {
            pattern: other.pattern.or(self.pattern.clone()),
            diffuse: other.diffuse.or(self.diffuse),
            ambient: other.ambient.or(self.ambient),
            specular: other.specular.or(self.specular),
            shininess: other.shininess.or(self.shininess),
            reflective: other.reflective.or(self.reflective),
            transparency: other.transparency.or(self.transparency),
            refractive_index: other.refractive_index.or(self.refractive_index),
            cast_shadows: other.cast_shadows.or(self.cast_shadows),
            receive_shadows: other.receive_shadows.or(self.receive_shadows),
        }
    }
}

impl IntoWithDefines<Material> for YamlMaterial {
    fn into_with_defines(self, defines: &[Define]) -> Material {
        let material = Material::new();
        let material = [
            self.pattern.map(|pattern| {
                Box::new(move |m: Material| m.with_pattern(pattern.into_with_defines(defines)))
                    as Box<dyn FnOnce(Material) -> Material>
            }),
            self.diffuse.map(|diffuse| {
                Box::new(move |m: Material| m.with_diffuse(diffuse))
                    as Box<dyn FnOnce(Material) -> Material>
            }),
            self.ambient.map(|ambient| {
                Box::new(move |m: Material| m.with_ambient(ambient))
                    as Box<dyn FnOnce(Material) -> Material>
            }),
            self.specular.map(|specular| {
                Box::new(move |m: Material| m.with_specular(specular))
                    as Box<dyn FnOnce(Material) -> Material>
            }),
            self.shininess.map(|shininess| {
                Box::new(move |m: Material| m.with_shininess(shininess))
                    as Box<dyn FnOnce(Material) -> Material>
            }),
            self.reflective.map(|reflective| {
                Box::new(move |m: Material| m.with_reflective(reflective))
                    as Box<dyn FnOnce(Material) -> Material>
            }),
            self.transparency.map(|transparency| {
                Box::new(move |m: Material| m.with_transparency(transparency))
                    as Box<dyn FnOnce(Material) -> Material>
            }),
            self.refractive_index.map(|refractive_index| {
                Box::new(move |m: Material| m.with_refractive_index(refractive_index))
                    as Box<dyn FnOnce(Material) -> Material>
            }),
            self.cast_shadows.map(|cast_shadows| {
                Box::new(move |m: Material| m.with_cast_shadows(cast_shadows))
                    as Box<dyn FnOnce(Material) -> Material>
            }),
            self.receive_shadows.map(|receive_shadows| {
                Box::new(move |m: Material| m.with_receive_shadows(receive_shadows))
                    as Box<dyn FnOnce(Material) -> Material>
            }),
        ]
        .into_iter()
        .flatten()
        .fold(material, |m, f| f(m));
        material
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "kebab-case")]
enum YamlPattern {
    Test,
    Solid {
        color: [f64; 3],
    },
    Striped {
        a: Box<YamlPattern>,
        b: Box<YamlPattern>,
        transform: Option<Vec<TransformOrReference>>,
    },
    Checker {
        a: Box<YamlPattern>,
        b: Box<YamlPattern>,
        transform: Option<Vec<TransformOrReference>>,
    },
    Ring {
        a: Box<YamlPattern>,
        b: Box<YamlPattern>,
        transform: Option<Vec<TransformOrReference>>,
    },
    LinearGradient {
        a: [f64; 3],
        b: [f64; 3],
        transform: Option<Vec<TransformOrReference>>,
    },
    Blend {
        a: Box<YamlPattern>,
        b: Box<YamlPattern>,
        transform: Option<Vec<TransformOrReference>>,
    },
    Perturbed {
        p: Box<YamlPattern>,
        transform: Option<Vec<TransformOrReference>>,
    },
}

impl IntoWithDefines<Pattern> for YamlPattern {
    fn into_with_defines(self, defines: &[Define]) -> Pattern {
        match self {
            YamlPattern::Test => Pattern::new_test_pattern(),
            YamlPattern::Solid { color } => Pattern::new_solid_pattern(color.into()),
            YamlPattern::Striped { a, b, transform } => {
                let mut pattern = Pattern::new_striped_pattern(
                    a.into_with_defines(defines),
                    b.into_with_defines(defines),
                );
                if let Some(transform) = transform {
                    pattern = pattern.with_transform(transform.into_with_defines(defines));
                }
                pattern
            }
            YamlPattern::Checker { a, b, transform } => {
                let mut pattern = Pattern::new_checker_pattern(
                    a.into_with_defines(defines),
                    b.into_with_defines(defines),
                );
                if let Some(transform) = transform {
                    pattern = pattern.with_transform(transform.into_with_defines(defines));
                }
                pattern
            }
            YamlPattern::Ring { a, b, transform } => {
                let mut pattern = Pattern::new_ring_pattern(
                    a.into_with_defines(defines),
                    b.into_with_defines(defines),
                );
                if let Some(transform) = transform {
                    pattern = pattern.with_transform(transform.into_with_defines(defines));
                }
                pattern
            }
            YamlPattern::LinearGradient { a, b, transform } => {
                let mut pattern = Pattern::new_linear_gradient(a.into(), b.into());
                if let Some(transform) = transform {
                    pattern = pattern.with_transform(transform.into_with_defines(defines));
                }
                pattern
            }
            YamlPattern::Blend { a, b, transform } => {
                let mut pattern = Pattern::new_blending_pattern(
                    a.into_with_defines(defines),
                    b.into_with_defines(defines),
                );
                if let Some(transform) = transform {
                    pattern = pattern.with_transform(transform.into_with_defines(defines));
                }
                pattern
            }
            YamlPattern::Perturbed { p, transform } => {
                let mut pattern = Pattern::new_perturbed_pattern(p.into_with_defines(defines));
                if let Some(transform) = transform {
                    pattern = pattern.with_transform(transform.into_with_defines(defines));
                }
                pattern
            }
        }
    }
}

impl IntoWithDefines<Pattern> for Box<YamlPattern> {
    fn into_with_defines(self, defines: &[Define]) -> Pattern {
        (*self).into_with_defines(defines)
    }
}

type DefinitionId = String;

pub struct YamlLoader {
    camera: Camera,
    lights: Vec<PointLight>,
    objects: Vec<Object>,
}

fn extract_commands(
    scene: Vec<SceneCommand>,
) -> (
    Option<YamlCamera>,
    Vec<YamlLight>,
    Vec<YamlObject>,
    Vec<Define>,
) {
    scene.into_iter().fold(
        (None, Vec::new(), Vec::new(), Vec::new()),
        |(mut camera, mut lights, mut objects, mut defines), command| {
            match command {
                SceneCommand::Add(Add::AddCamera(c)) => camera = Some(c),
                SceneCommand::Add(Add::AddLight(l)) => lights.push(l),
                SceneCommand::Add(Add::AddObject(o)) => objects.push(o),
                SceneCommand::Define(d) => defines.push(d),
            }
            (camera, lights, objects, defines)
        },
    )
}

impl YamlLoader {
    pub fn from(path: &Path) -> Self {
        let yaml_str = fs::read_to_string(path).unwrap();
        let scene: Vec<SceneCommand> = serde_yml::from_str(yaml_str.as_str()).unwrap();

        let (camera, lights, objects, defines) = extract_commands(scene);
        let defines: Vec<Define> = defines.iter().map(|def| def.expand(&defines)).collect();
        let camera: Camera = camera.unwrap().into();
        let lights: Vec<PointLight> = lights.into_iter().map(Into::into).collect();
        let objects: Vec<Object> = objects
            .into_iter()
            .map(|o| o.into_with_defines(&defines))
            .collect();
        Self {
            camera,
            lights,
            objects,
        }
    }

    pub fn to_ppm(&self, path: &Path) {
        let w = World::new()
            .with_lights(self.lights.clone())
            .with_objects(self.objects.clone());
        println!("{:#?}", &w);
        let canvas = self.camera.render(w);
        let ppm = PPM::from(canvas);
        fs::write(path, ppm.to_string()).unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deserialize() {
        let yml_str = r#"
- add: camera
  width: 100
  height: 100
  field-of-view: 0.785
  from: [ -6, 0, -10 ]
  to: [ 6, 0, 6 ]
  up: [ -0.45, 1, 0 ]
- add: cone
  min: 0.32
  max: 12
  cap: bottom-cap
- add: sphere
  material: 
    pattern: 
      kind: checker
      a:  
        kind: solid
        color: [1.0, 0.5, 0.3]
      b:
        kind: striped
        a: 
          kind: solid
          color: [0.1, 0.2, 0.3]
        b: 
          kind: solid
          color: [0.4, 0.5, 0.6]
  transform: 
  - [scale, 0.1, 0.1, 0.1]
  - pippo
- define: my-def
  extend: some-other-def
  transform:
  - [rotate-y, 0.345]
  - [scale, 0.5, 0.5, 0.5]
  - [translate, 0, 1, 0]
  material:
    pattern:
      kind: checker
      a: 
        kind: solid
        color: [0, 0, 0]
      b:
        kind: solid
        color: [1, 1, 1]
- add: group
  children: 
  - add: cube
  - add: sphere
    transform:
    - [rotate-x, 12]
- add: csg
  kind: union
  left:
    add: cube
  right:
    add: sphere
    transform:
    - ciao
  "#;
        let commands: Vec<SceneCommand> = serde_yml::from_str(yml_str).unwrap();
        for command in commands {
            println!("{:?}", command);
        }
    }

    #[test]
    fn yaml_materials_can_be_merged() {
        let mat_1 = YamlMaterial {
            pattern: Some(YamlPattern::Solid {
                color: [1.0, 1.0, 1.0],
            }),
            diffuse: Some(10.0),
            ambient: None,
            specular: Some(1.0),
            shininess: None,
            reflective: Some(11.0),
            transparency: None,
            refractive_index: Some(1.11),
            cast_shadows: None,
            receive_shadows: Some(true),
        };
        let mat_2 = YamlMaterial {
            pattern: Some(YamlPattern::Solid {
                color: [0.404, 0.01, 0.9],
            }),
            diffuse: None,
            ambient: Some(20.0),
            specular: Some(2.0),
            shininess: None,
            reflective: None,
            transparency: Some(22.22),
            refractive_index: Some(2.22),
            cast_shadows: Some(true),
            receive_shadows: Some(false),
        };
        let mat_3 = mat_1.merge(mat_2);
        let expected = YamlMaterial {
            pattern: Some(YamlPattern::Solid {
                color: [0.404, 0.01, 0.9],
            }),
            diffuse: Some(10.0),
            ambient: Some(20.0),
            specular: Some(2.0),
            shininess: None,
            reflective: Some(11.0),
            transparency: Some(22.22),
            refractive_index: Some(2.22),
            cast_shadows: Some(true),
            receive_shadows: Some(false),
        };
        assert_eq!(mat_3, expected);
    }

    #[test]
    fn defines_can_be_inherited() {
        let yml_str = r#"
- define: white-material
  material:
    pattern:
      kind: solid
      color: [ 1, 1, 1 ]
    diffuse: 0.7
    ambient: 0.1
    specular: 0.0
    reflective: 0.1
- define: blue-material
  extend: white-material
  material:
    pattern: 
      kind: solid
      color: [ 0.537, 0.831, 0.914 ]
        "#;
        let commands: Vec<SceneCommand> = serde_yml::from_str(yml_str).unwrap();
        let (_, _, _, defines) = extract_commands(commands);
        assert_eq!(defines.len(), 2);
        let white_material = Define {
            define: "white-material".to_string(),
            extend: None,
            transform: None,
            material: Some(YamlMaterial {
                pattern: Some(YamlPattern::Solid {
                    color: [1.0, 1.0, 1.0],
                }),
                diffuse: Some(0.7),
                ambient: Some(0.1),
                specular: Some(0.0),
                shininess: None,
                reflective: Some(0.1),
                transparency: None,
                refractive_index: None,
                cast_shadows: None,
                receive_shadows: None,
            }),
        };
        let blue_material = Define {
            define: "blue-material".to_string(),
            extend: None,
            transform: None,
            material: Some(YamlMaterial {
                pattern: Some(YamlPattern::Solid {
                    color: [0.537, 0.831, 0.914],
                }),
                diffuse: Some(0.7),
                ambient: Some(0.1),
                specular: Some(0.0),
                shininess: None,
                reflective: Some(0.1),
                transparency: None,
                refractive_index: None,
                cast_shadows: None,
                receive_shadows: None,
            }),
        };
        let defines: Vec<Define> = defines.iter().map(|def| def.expand(&defines)).collect();
        assert_eq!(defines[0], white_material);
        assert_eq!(defines[1], blue_material);
    }
}
