use image::{GenericImageView, Rgba};
use std::fs;

const LUMINANCE_THRESHOLD_1: f64 = 200.0;
const LUMINANCE_THRESHOLD_2: f64 = 100.0;
const LUMINANCE_THRESHOLD_3: f64 = 50.0;

fn image_to_ascii(img: image::DynamicImage) {
    println!("Converting to ASCII :\n");

    let mut ascii_string = String::new();
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let ascii_char = convert_pixel_to_ascii(&pixel);
            ascii_string.push(ascii_char);
        }
        ascii_string.push('\n');
    }
    println!("{}", ascii_string);
}

fn convert_pixel_to_ascii(pixel: &Rgba<u8>) -> char {
    let luminance = 0.2126 * pixel[0] as f64 + 0.7152 * pixel[1] as f64 + 0.0722 * pixel[2] as f64;

    match luminance {
        _ if luminance > LUMINANCE_THRESHOLD_1 => ' ',
        _ if luminance > LUMINANCE_THRESHOLD_2 => '.',
        _ if luminance > LUMINANCE_THRESHOLD_3 => ':',
        _ => '#',
    }
}

fn main() {
    println!("Welcome to the ASCII image converter, I did it quickly and it stinks of shit");

    println!("send your path to your image :");
    let mut img_path = String::new();
    std::io::stdin().read_line(&mut img_path).unwrap();
    img_path = img_path.trim().to_string();

    match fs::metadata(&img_path) {
        Ok(_) => {
            match image::open(&img_path) {
                Ok(img) => image_to_ascii(img),
                Err(e) => eprintln!("I couldn't open your image {}", e),
            }
        },
        Err(e) => eprintln!("I couldn't find your file {}", e),
    }
}
