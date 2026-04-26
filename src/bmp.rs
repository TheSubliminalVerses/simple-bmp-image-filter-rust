use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom};

macro_rules! load_struct_data {
    ($head:ident, $($tail:expr),+) => {
        {
            let mut bytes = 0;
            let mut reader = BufReader::new($head);
            $(bytes += reader.read(&mut $tail).unwrap();)*
            bytes
        }
    };
}

pub struct BMPFileHeader {
    bf_type: [u8; 2],
    bf_size: [u8; 4],
    bf_reserved1: [u8; 2],
    bf_reserved2: [u8; 2],
    bf_off_bits: [u8; 4],
}

pub struct BMPInfoHeader {
    bi_size: [u8; 4],
    bi_width: [u8; 4],
    bi_height: [u8; 4],
    bi_bit_count: [u8; 2],
    bi_compression: [u8; 4],
    bi_size_image: [u8; 4],
    bi_x_pixels_per_meter: [u8; 4],
    bi_y_pixels_per_meter: [u8; 4],
    bi_crl_used: [u8; 4],
    bi_crl_important: [u8; 4],
}

pub struct Pixel {
    blue: [u8; 1],
    green: [u8; 1],
    red: [u8; 1],
}

impl BMPFileHeader {
    pub fn new() -> BMPFileHeader {
        BMPFileHeader {
            bf_type: [0; 2],
            bf_size: [0; 4],
            bf_reserved1: [0; 2],
            bf_reserved2: [0; 2],
            bf_off_bits: [0; 4],
        }
    }
    
    pub fn read_from_file(&mut self, file: &File) -> usize {
        let bytes = load_struct_data!(file, self.bf_type, self.bf_size, self.bf_reserved1, self.bf_reserved2,
            self.bf_off_bits);
        bytes
    }
    
    pub fn get_type(&self) -> u16 {
        u16::from_le_bytes(self.bf_type)
    }
}

impl BMPInfoHeader {
    pub fn new() -> BMPInfoHeader {
        BMPInfoHeader {
            bi_size: [0; 4],
            bi_width: [0; 4],
            bi_height: [0; 4],
            bi_bit_count: [0; 2],
            bi_compression: [0; 4],
            bi_size_image: [0; 4],
            bi_x_pixels_per_meter: [0; 4],
            bi_y_pixels_per_meter: [0; 4],
            bi_crl_used: [0; 4],
            bi_crl_important: [0; 4],
        }
    }

    pub fn read_from_file(&mut self, file: &File) -> usize {
        let bytes = load_struct_data!(file, self.bi_size, self.bi_width,self.bi_height,
        self.bi_bit_count, self.bi_compression, self.bi_size_image, self.bi_x_pixels_per_meter,
        self.bi_y_pixels_per_meter, self.bi_crl_used, self.bi_crl_important);
        bytes
    }
    
    pub fn get_dim(&self) -> (i32, i32) {
        (i32::from_le_bytes(self.bi_width), i32::from_le_bytes(self.bi_height))
    }
}

impl Pixel {
    pub fn new(blue: [u8; 1], green: [u8; 1], red: [u8; 1]) -> Pixel {
        Pixel { blue, green, red, }
    }
    
    pub fn set_blue(&mut self, blue: u8) {
        self.blue = u8::to_le_bytes(blue);
    }
    
    pub fn set_green(&mut self, green: u8) {
        self.green = u8::to_le_bytes(green);
    }
    
    pub fn set_red(&mut self, red: u8) {
        self.red = u8::to_le_bytes(red);
    }
    
    pub fn get_blue(&self) -> u8 {
        u8::from_le_bytes(self.blue)
    }
    
    pub fn get_green(&self) -> u8 {
        u8::from_le_bytes(self.green)
    }
    
    pub fn get_red(&self) -> u8 {
        u8::from_le_bytes(self.red)
    }
}