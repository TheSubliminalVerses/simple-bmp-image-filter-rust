use std::fs::{File};
use std::io::{Seek, SeekFrom, BufReader, Read, Write};
use crate::bmp::{BMPFileHeader, BMPInfoHeader, Pixel};
use crate::helpers::to_grayscale;

mod bmp;
mod helpers;

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
        Ok(file) => file,
        Err(why) => panic!("Error opening file: {}", why),
    };

    let mut bytes: usize = 0;

    let mut bfh = BMPFileHeader::new();
    let mut bih = BMPInfoHeader::new();

    bytes += bfh.read_from_file(&file);

    match file.seek(SeekFrom::Start(bytes as u64)) {
        Ok(_) => {}
        Err(why) => panic!("Error reading file: {}", why),
    }

    bytes += bih.read_from_file(&file);

    match file.seek(SeekFrom::Start(bytes as u64)) {
        Ok(_) => {}
        Err(why) => panic!("Error reading file: {}", why),
    }

    let width = bih.get_dim().0.abs() as usize;
    let height = bih.get_dim().1.abs() as usize;

    let mut image: Vec<Vec<Pixel>> = Vec::with_capacity(height);

    let mut reader = BufReader::new(&file);

    for y in 0..height {
        let mut row: Vec<Pixel> = Vec::with_capacity(width);
        for _ in 0..width {
            let mut blue: [u8; 1] = [0; 1];
            let mut green: [u8; 1] = [0; 1];
            let mut red: [u8; 1] = [0; 1];

            match reader.read_exact(&mut blue) {
                Ok(_) => {},
                Err(why) => panic!("Error reading file: {}", why),
            }

            match reader.read_exact(&mut green) {
                Ok(_) => {},
                Err(why) => panic!("Error reading file: {}", why),
            }

            match reader.read_exact(&mut red) {
                Ok(_) => {},
                Err(why) => panic!("Error reading file: {}", why),
            }

            row.push(Pixel::new(blue, green, red));
        }
        image.push(row);
    }

    match flag.as_str() {
        "-g" => to_grayscale(width, height, &mut image),
        _ => panic!("Invalid file flag: {}", flag),
    }
    

    println!("DONE!");
}
