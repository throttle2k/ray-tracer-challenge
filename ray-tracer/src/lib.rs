use std::sync::RwLock;

use lazy_static::lazy_static;
use registry::Registry;

pub mod camera;
pub mod canvas;
pub mod intersections;
pub mod lights;
pub mod materials;
pub mod matrix;
pub mod patterns;
pub mod ppm;
pub mod rays;
pub mod registry;
pub mod shapes;
pub mod transformations;
pub mod tuples;
pub mod world;

lazy_static! {
    pub static ref REGISTRY: RwLock<Registry> = RwLock::new(Registry::new());
}
