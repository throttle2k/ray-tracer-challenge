use std::{env, path::PathBuf};

use clap::Parser;
use ray_tracer::yaml_loader::YamlLoader;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short = 's', long, default_value=get_default_scenes_path().into_os_string())]
    scenes_path: PathBuf,
    #[arg(short = 'r', long, default_value=get_default_resources_path().into_os_string())]
    resources_path: PathBuf,
    #[arg(short = 't', long, default_value=get_default_target_path().into_os_string())]
    target_path: PathBuf,
    #[arg(short = 'f', long)]
    scene_file_name: String,
    #[arg(short = 'o', long)]
    output_file_name: String,
}

fn main() {
    let args = Args::parse();
    let mut scene = args.scenes_path.clone();
    scene.push(args.scene_file_name);
    let mut output = args.target_path.clone();
    output.push(args.output_file_name);
    let loader = YamlLoader::from(&scene);
    loader.to_ppm(&output);
}

fn get_default_path(folder: &str) -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push(folder);
    path
}

fn get_default_scenes_path() -> PathBuf {
    get_default_path("scenes")
}

fn get_default_resources_path() -> PathBuf {
    get_default_path("resources")
}

fn get_default_target_path() -> PathBuf {
    get_default_path("")
}
