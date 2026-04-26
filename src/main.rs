use std::fs::{File};
use std::io::{Seek, SeekFrom};
use crate::bmp::{BMPFileHeader, BMPInfoHeader};

mod bmp;

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


    println!("Magic Number: 0x{:x}", bfh.get_type());
    println!("Width: {}, Height: {}", bih.get_dim().0, bih.get_dim().1);
}
