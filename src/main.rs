use std::fs::File;
use handler::BMPFile;

mod bmp;
mod handler;

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
    let image_ref = bmp_file.get_image_ref_as_mut();

    for y in 0..1 {
        for x in 0..4 {
            println!("Blue: 0x{:x}, Green: 0x{:x}, Red: 0x{:x}",
            image_ref[y][x].get_blue(), image_ref[y][x].get_green(), image_ref[y][x].get_red());
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
