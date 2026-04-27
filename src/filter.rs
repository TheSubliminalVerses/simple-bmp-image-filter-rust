use crate::bmp::{Pixel};

pub fn to_grayscale(width: usize, height: usize, image: &mut Vec<Vec<Pixel>>) {
    for y in 0..height {
        for x in 0..width {
            let gray: u32 = (image[y][x].get_blue() as u32 + image[y][x].get_green() as u32
            + image[y][x].get_red() as u32) / 3;

            let capped_gray = gray.min(255);

            image[y][x].set_blue(capped_gray as u8);
            image[y][x].set_green(capped_gray as u8);
            image[y][x].set_red(capped_gray as u8);
        }
    }
}

pub fn to_sepia(width: usize, height: usize, image: &mut Vec<Vec<Pixel>>) {
    for y in 0..height {
        for x in 0..width {
            let new_red: f32 = (image[y][x].get_red() as f32 * 0.393)
                + (image[y][x].get_green() as f32 * 0.769)
                + (image[y][x].get_blue() as f32 * 0.189);

            let new_green: f32 = (image[y][x].get_red() as f32 * 0.349)
                + (image[y][x].get_green() as f32 * 0.686)
                + (image[y][x].get_blue() as f32 * 0.168);

            let new_blue: f32 = (image[y][x].get_red() as f32 * 0.272)
                + (image[y][x].get_green() as f32 * 0.534)
                + (image[y][x].get_blue() as f32 * 0.131);

            let capped_red = new_red.min(255.0);
            let capped_green = new_green.min(255.0);
            let capped_blue = new_blue.min(255.0);

            image[y][x].set_blue(capped_blue as u8);
            image[y][x].set_green(capped_green as u8);
            image[y][x].set_red(capped_red as u8);
        }
    }
}