#![allow(unused_parens)]

extern crate image;
extern crate mime_guess;
use image::{ImageBuffer, Rgba};
use std::fs;
use std::path::PathBuf;

const INPUT_DIR: &str = "./input";
const OUTPUT_DIR: &str = "./output";

const RED_MULTIPLIER: f32 = 0.859;
const GREEN_MULTIPLIER: f32 = 0.812;
const BLUE_MULTIPLIER: f32 = 0.808;

fn get_input_images() -> Vec<PathBuf> {
    let mut image_paths: Vec<PathBuf> = Vec::new();

    let entries = fs::read_dir(INPUT_DIR).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        
        // check if entry is an image
        if(!path.is_file()) { continue; }
        let mime = mime_guess::from_path(&path).first_or_octet_stream();
        if(!mime.type_().as_str().starts_with("image")) { continue; }

        image_paths.push(path);
    }

    return image_paths;
}

fn process_image(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut new_image = ImageBuffer::new(image.width(), image.height());

    for (x, y, pixel) in image.enumerate_pixels() {
        let mut new_pixel = *pixel;

        new_pixel[0] = (new_pixel[0] as f32 * RED_MULTIPLIER) as u8;
        new_pixel[1] = (new_pixel[1] as f32 * GREEN_MULTIPLIER) as u8;
        new_pixel[2] = (new_pixel[2] as f32 * BLUE_MULTIPLIER) as u8;

        new_image.put_pixel(x, y, new_pixel);
    }

    return new_image;
}

fn main() {
    if(!PathBuf::from(INPUT_DIR).is_dir()) {
        println!("Input directory '{}' does not exist; creating directory...", INPUT_DIR);
        fs::create_dir(INPUT_DIR).unwrap();
        println!("Please add images to the directory and run the program again.");
        return;
    }
    if(fs::read_dir(INPUT_DIR).unwrap().next().is_none()) {
        println!("Input directory '{}' is empty; please add images to the directory and run the program again.", INPUT_DIR);
        return;
    }
    if(!PathBuf::from(OUTPUT_DIR).is_dir()) {
        fs::create_dir(OUTPUT_DIR).unwrap();
    }

    let image_paths = get_input_images();

    for (i, path) in image_paths.iter().enumerate() {
        let image = image::open(path.clone()).unwrap().to_rgba8();
        let new_image = process_image(image);

        let name = path.file_name().unwrap().to_str().unwrap();
        let output_path = format!("{}/{}", OUTPUT_DIR, name);
        new_image.save(output_path).unwrap();

        println!("{} processed - {}% complete ({}/{})", name, (i as f32 / image_paths.len() as f32 * 100.0).round(), i + 1, image_paths.len());
    }
    println!("Image processing complete! You can find the processed images in the output directory ({}).", OUTPUT_DIR);
}