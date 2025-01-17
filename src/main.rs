use std::{fs::File, path::Path};

use image::GenericImageView;

const RGBA_RED_INDEX: usize = 0;
const RGBA_GREEN_INDEX: usize = 1;
const RGBA_BLUE_INDEX: usize = 2;
const RGBA_ALPHA_INDEX: usize = 3;

fn main() {
    println!("Hello, world!");

    create_normal_texture();
    create_specular_texture();
}

fn create_normal_texture() {
    let normal_map = image::open(Path::new(todo!())).unwrap();
    let ambient_occlusion_map = image::open(Path::new(todo!())).unwrap();
    let height_map = image::open(Path::new(todo!())).unwrap();
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

    let mut normal_texture_file =
        File::create("OUTPUT_TODO").expect("should be able to create normal texture (_n) file");
    normal_map
        .write_to(&mut normal_texture_file, image::ImageFormat::Png)
        .expect("should be able to write to normal texture (_n) file");
}

fn create_specular_texture() {
    let smoothness_map = image::open(Path::new(todo!())).unwrap();
    let reflectance_map = image::open(Path::new(todo!())).unwrap();
    let emisiveness_map = image::open(Path::new(todo!())).unwrap();
    for smoothness_map_pixel in smoothness_map.pixels() {
        let mut smoothness_map_pixel_rgba = smoothness_map_pixel.2.0;
        // Store the reflectance value in the green channel (0-229)
        smoothness_map_pixel_rgba[RGBA_GREEN_INDEX] = reflectance_map
            .get_pixel(smoothness_map_pixel.0, smoothness_map_pixel.1)
            .0[RGBA_RED_INDEX];
        // TODO: Add support for porosity maps and subsurface scattering maps in the blue channel
        smoothness_map_pixel_rgba[RGBA_BLUE_INDEX] = 0;
        // Store the emisiveness value in the alpha channel (0-254)
        smoothness_map_pixel_rgba[RGBA_ALPHA_INDEX] = emisiveness_map
            .get_pixel(smoothness_map_pixel.0, smoothness_map_pixel.1)
            .0[RGBA_RED_INDEX];
    }
}
