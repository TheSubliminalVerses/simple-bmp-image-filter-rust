use std::fs::File;

mod bmp;
mod handler;
mod filter;

use handler::BMPFile;
use crate::filter::{box_blur, edge_detection, to_grayscale, to_sepia};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let argc = args.len();

    if argc < 4 {
        println!("Not enough arguments!");
        return;
    }

    if argc > 4 {
        println!("Too many arguments!");
        return;
    }

    let flag = &args[1];
    let infile = &args[2];
    let outfile = &args[3];

    let mut file = match File::open(infile) {
        Ok(f) => f,
        Err(e) => panic!("Error: {}", e),
    };

    let mut bmp_file = BMPFile::new();
    bmp_file.read_headers(&mut file);

    bmp_file.read_image_data(&mut file);

    let width = bmp_file.get_bih().get_dim().0.abs() as usize;
    let height = bmp_file.get_bih().get_dim().1.abs() as usize;

    let image_ref = bmp_file.get_image_ref_as_mut();

    match flag.as_str() {
        "-g" => to_grayscale(width, height, image_ref),
        "-s" => to_sepia(width, height, image_ref),
        "-b" => box_blur(width, height, 1, image_ref),
        "-e" => edge_detection(width, height, 1, image_ref),
        _ => {
            panic!("Invalid flag! {}", flag);
        }
    }

    let mut out = match File::create(outfile) {
        Ok(f) => f,
        Err(e) => panic!("Error: {}", e),
    };

    bmp_file.write_headers(&mut out);
    bmp_file.write_image_data(&mut out);

    println!("BMP file written");
}
