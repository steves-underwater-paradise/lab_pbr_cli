#![feature(path_file_prefix)]

use std::{
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

use clap::Parser;
use image::GenericImageView;
use lab_pbr_cli::cli::arguments::{ARGUMENTS, Arguments};

const RGBA_RED_INDEX: usize = 0;
const RGBA_GREEN_INDEX: usize = 1;
const RGBA_BLUE_INDEX: usize = 2;
const RGBA_ALPHA_INDEX: usize = 3;
const PNG_FILE_EXTENSION: &str = "png";

enum TextureType {
    Normal,
    Specular,
}

fn main() -> Result<(), std::io::Error> {
    ARGUMENTS.set(Arguments::parse()).unwrap();
    let cli_arguments = ARGUMENTS
        .get()
        .expect("should be able to parse CLI arguments");
    create_normal_texture(cli_arguments);
    create_specular_texture(cli_arguments);

    Ok(())
}

fn create_normal_texture(cli_arguments: &Arguments) {
    let normal_map = image::open(&cli_arguments.normal_map_path).unwrap();
    let ambient_occlusion_map = image::open(&cli_arguments.ambient_occlusion_map_path).unwrap();
    let height_map = image::open(&cli_arguments.height_map_path).unwrap();
    for normal_map_pixel in normal_map.pixels() {
        let mut normal_map_pixel_rgba = normal_map_pixel.2.0;
        // Store the ambient occlusion value in the blue channel
        normal_map_pixel_rgba[RGBA_BLUE_INDEX] = ambient_occlusion_map
            .get_pixel(normal_map_pixel.0, normal_map_pixel.1)
            .0[RGBA_RED_INDEX];
        // Store the height value in the alpha channel
        normal_map_pixel_rgba[RGBA_ALPHA_INDEX] = height_map
            .get_pixel(normal_map_pixel.0, normal_map_pixel.1)
            .0[RGBA_RED_INDEX];
    }

    let normal_texture_path = get_output_texture_path(
        cli_arguments
            .diffuse_map_path
            .file_prefix()
            .expect("should be able to get diffuse map file name"),
        TextureType::Normal,
        &cli_arguments.output_directory,
    );
    println!(
        "Saving normal texture (_n) to path: {:?}",
        normal_texture_path
    );

    let mut normal_texture_file = File::create(&normal_texture_path)
        .expect("should be able to create normal texture (_n) file");
    normal_map
        .write_to(&mut normal_texture_file, image::ImageFormat::Png)
        .expect("should be able to write to normal texture (_n) file");
    println!("Saved normal texture (_n).");
}

fn create_specular_texture(cli_arguments: &Arguments) {
    let smoothness_map = image::open(&cli_arguments.smoothness_map).unwrap();
    let reflectance_map = image::open(&cli_arguments.reflectance_map).unwrap();
    let emissiveness_map = image::open(&cli_arguments.emissiveness_map).unwrap();
    for smoothness_map_pixel in smoothness_map.pixels() {
        let mut smoothness_map_pixel_rgba = smoothness_map_pixel.2.0;
        // Store the reflectance value in the green channel (0-229)
        smoothness_map_pixel_rgba[RGBA_GREEN_INDEX] = reflectance_map
            .get_pixel(smoothness_map_pixel.0, smoothness_map_pixel.1)
            .0[RGBA_RED_INDEX];
        // TODO: Add support for porosity maps and subsurface scattering maps in the blue channel
        smoothness_map_pixel_rgba[RGBA_BLUE_INDEX] = 0;
        // Store the emisiveness value in the alpha channel (0-254)
        smoothness_map_pixel_rgba[RGBA_ALPHA_INDEX] = emissiveness_map
            .get_pixel(smoothness_map_pixel.0, smoothness_map_pixel.1)
            .0[RGBA_RED_INDEX];
    }

    let specular_texture_path = get_output_texture_path(
        cli_arguments
            .diffuse_map_path
            .file_prefix()
            .expect("should be able to get diffuse map file name"),
        TextureType::Specular,
        &cli_arguments.output_directory,
    );
    println!(
        "Saving specular texture (_n) to path: {:?}",
        specular_texture_path
    );

    let mut specular_texture_file = File::create(specular_texture_path)
        .expect("should be able to create specular texture (_s) file");
    smoothness_map
        .write_to(&mut specular_texture_file, image::ImageFormat::Png)
        .expect("should be able to write to specular texture (_s) file");
    println!("Saved specular texture (_s).");
}

fn get_output_texture_path(
    texture_name: &OsStr,
    texture_type: TextureType,
    output_directory: &Path,
) -> PathBuf {
    // TODO: Implement Display for TextureType instead
    let texture_type_name = match texture_type {
        TextureType::Normal => "_n",
        TextureType::Specular => "_s",
    };
    let mut texture_name_with_type_suffix = texture_name.to_owned();
    texture_name_with_type_suffix.push(texture_type_name);

    let mut output_texture_path = PathBuf::from(output_directory).join(texture_name_with_type_suffix);
    output_texture_path.set_extension(PNG_FILE_EXTENSION);

    output_texture_path
}
