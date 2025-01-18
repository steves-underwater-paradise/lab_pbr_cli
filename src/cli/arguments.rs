use std::{env, path::PathBuf, sync::OnceLock};

use clap::{Parser, crate_version};

#[derive(Debug, Default, Parser)]
#[command(version = crate_version!(), about, long_about = None)]
pub struct Arguments {
    #[arg(short, long, short_alias('i'), alias("input-texture-path"))]
    pub diffuse_map_path: PathBuf,
    #[arg(short, long)]
    pub normal_map_path: PathBuf,
    #[arg(short, long)]
    pub ambient_occlusion_map_path: PathBuf,
    #[arg(long)]
    pub height_map_path: PathBuf,
    #[arg(short, long)]
    pub smoothness_map_path: PathBuf,
    #[arg(short, long)]
    pub reflectance_map_path: PathBuf,
    #[arg(short, long)]
    pub emissiveness_map_path: PathBuf,
    #[arg(short, long, default_value = env::current_dir().expect("should be able to get current working directory").into_os_string())]
    pub output_directory: PathBuf,
}

pub static ARGUMENTS: OnceLock<Arguments> = OnceLock::new();
