use std::fs::{File};
use std::io::{Read, Write};
use crate::bmp::{BMPFileHeader, BMPInfoHeader, Pixel};

macro_rules! read_struct_data {
    ($head:ident, $($tail:expr),+) => {
        {
            $($head.read_exact(&mut $tail).unwrap();)*
        }
    };
}

macro_rules! write_struct_data {
    ($head:ident, $($tail:expr),+) => {
        {
            $($head.write(&$tail).unwrap();)*
        }
    };
}

pub struct BMPFile {
    bfh: BMPFileHeader,
    bih: BMPInfoHeader,
    image: Vec<Vec<Pixel>>
}

impl BMPFile {
    pub fn new() -> BMPFile {
        BMPFile {
            bfh: BMPFileHeader::new(),
            bih: BMPInfoHeader::new(),
            image: Vec::new()
        }
    }

    pub fn read_headers(&mut self, file: &mut File) {
        read_struct_data!(file, self.bfh.bf_type, self.bfh.bf_size, self.bfh.bf_reserved1,
        self.bfh.bf_reserved2, self.bfh.bf_off_bits);

        read_struct_data!(file, self.bih.bi_size, self.bih.bi_width, self.bih.bi_height,
            self.bih.bi_planes, self.bih.bi_bit_count, self.bih.bi_compression,
            self.bih.bi_size_image, self.bih.bi_x_pixels_per_meter, self.bih.bi_y_pixels_per_meter,
            self.bih.bi_crl_used, self.bih.bi_crl_important);
    }

    pub fn write_headers(&self, file: &mut File) {
        write_struct_data!(file, self.bfh.bf_type, self.bfh.bf_size, self.bfh.bf_reserved1,
        self.bfh.bf_reserved2, self.bfh.bf_off_bits);

        write_struct_data!(file, self.bih.bi_size, self.bih.bi_width, self.bih.bi_height,
            self.bih.bi_planes, self.bih.bi_bit_count, self.bih.bi_compression,
            self.bih.bi_size_image, self.bih.bi_x_pixels_per_meter, self.bih.bi_y_pixels_per_meter,
            self.bih.bi_crl_used, self.bih.bi_crl_important);
    }

    pub fn read_image_data(&mut self, file: &mut File) {
        let width = self.bih.get_dim().0.abs() as usize;
        let height = self.bih.get_dim().1.abs() as usize;
        let bit_count = u16::from_le_bytes(self.bih.bi_bit_count);

        let stride = ((((bit_count as usize) * width) + 31) / 32) * 4;

        for _ in 0..height {
            let mut row: Vec<Pixel> = Vec::with_capacity(width);
            for _ in (0..stride).step_by(3) {
                let mut blue: [u8; 1] = [0; 1];
                let mut green: [u8; 1] = [0; 1];
                let mut red: [u8; 1] = [0; 1];

                read_struct_data!(file, blue, green, red);

                row.push(Pixel::new(blue, green, red));
            }
            self.image.push(row);
        }
    }

    pub fn write_image_data(&self, file: &mut File) {
        let width = self.bih.get_dim().0.abs() as usize;
        let height = self.bih.get_dim().1.abs() as usize;
        let bit_count = u16::from_le_bytes(self.bih.bi_bit_count);

        let stride = ((((bit_count as usize) * width) + 31) / 32) * 4;

        for y in 0..height {
            for x in (0..stride).step_by(3) {
                write_struct_data!(file, self.image[y][x / 3].blue, self.image[y][x / 3].green,
                    self.image[y][x / 3].red);
            }
        }
    }

    pub fn get_bfh(&self) -> &BMPFileHeader {
        &self.bfh
    }

    pub fn get_bih(&self) -> &BMPInfoHeader {
        &self.bih
    }

    pub fn get_image_ref_as_mut(&mut self) -> &mut Vec<Vec<Pixel>> {
        &mut self.image
    }
}