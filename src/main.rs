#![feature(path_file_prefix)]

use std::{
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

use clap::Parser;
use image::{DynamicImage, GenericImageView, RgbaImage};

use lab_pbr_cli::cli::arguments::{ARGUMENTS, Arguments};

const RGBA_RED_INDEX: usize = 0;
const RGBA_GREEN_INDEX: usize = 1;
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
    let texture_dimensions = image::open(&cli_arguments.diffuse_map_path)
        .expect("should be able to read the diffuse texture")
        .dimensions();
    create_normal_texture(cli_arguments, texture_dimensions);
    create_specular_texture(cli_arguments, texture_dimensions);

    Ok(())
}

fn create_normal_texture(cli_arguments: &Arguments, texture_dimensions: (u32, u32)) {
    let mut output_texture = RgbaImage::new(texture_dimensions.0, texture_dimensions.1);
    let normal_map = image::open(&cli_arguments.normal_map_path).unwrap();
    let ambient_occlusion_map = image::open(&cli_arguments.ambient_occlusion_map_path).unwrap();
    let height_map = if cli_arguments.height_map_path.is_some() {
        image::open(cli_arguments.height_map_path.clone().unwrap()).unwrap()
    } else {
        DynamicImage::ImageRgba8(RgbaImage::new(texture_dimensions.0, texture_dimensions.1))
    };
    for (x, y, output_pixel) in output_texture.enumerate_pixels_mut() {
        let normal_map_pixel_color = normal_map.get_pixel(x, y).0;
        output_pixel.0 = [
            // Store the normal values in the red and green channels
            normal_map_pixel_color[RGBA_RED_INDEX],
            normal_map_pixel_color[RGBA_GREEN_INDEX],
            // Store the ambient occlusion value in the blue channel
            ambient_occlusion_map.get_pixel(x, y).0[RGBA_RED_INDEX],
            // Store the height value in the alpha channel
            height_map.get_pixel(x, y).0[RGBA_RED_INDEX],
        ];
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
    output_texture
        .write_to(&mut normal_texture_file, image::ImageFormat::Png)
        .expect("should be able to write to normal texture (_n) file");
    println!("Saved normal texture (_n).");
}

fn create_specular_texture(cli_arguments: &Arguments, texture_dimensions: (u32, u32)) {
    let mut output_texture = RgbaImage::new(texture_dimensions.0, texture_dimensions.1);
    let smoothness_map = image::open(&cli_arguments.smoothness_map_path).unwrap();
    let reflectance_map = if cli_arguments.reflectance_map_path.is_some() {
        image::open(cli_arguments.reflectance_map_path.clone().unwrap()).unwrap()
    } else {
        DynamicImage::ImageRgba8(RgbaImage::new(texture_dimensions.0, texture_dimensions.1))
    };
    let emissiveness_map = if cli_arguments.emissiveness_map_path.is_some() {
        image::open(cli_arguments.emissiveness_map_path.clone().unwrap()).unwrap()
    } else {
        DynamicImage::ImageRgba8(RgbaImage::new(texture_dimensions.0, texture_dimensions.1))
    };
    for (x, y, output_pixel) in output_texture.enumerate_pixels_mut() {
        output_pixel.0 = [
            // Store the smoothness values in the red and green channels
            smoothness_map.get_pixel(x, y).0[RGBA_RED_INDEX],
            // Store the reflectance value in the green channel (0-229)
            // TODO: Clamp the reflectance value between 0-229
            reflectance_map.get_pixel(x, y).0[RGBA_GREEN_INDEX],
            // TODO: Add support for porosity maps and subsurface scattering maps in the blue channel
            0,
            // Store the height value in the alpha channel
            emissiveness_map.get_pixel(x, y).0[RGBA_RED_INDEX],
        ];
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
    output_texture
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

    let mut output_texture_path =
        PathBuf::from(output_directory).join(texture_name_with_type_suffix);
    output_texture_path.set_extension(PNG_FILE_EXTENSION);

    output_texture_path
}
