use std::path::Path;

use ray_tracer::yaml_loader::YamlLoader;

fn main() {
    let loader = YamlLoader::from(&Path::new("./samples/cover.yml"));
    loader.to_ppm(&Path::new("./cover.ppm"));
}
