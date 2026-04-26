use crate::bmp::Pixel;

pub fn to_grayscale(width: usize, height: usize, image: &mut Vec<Vec<Pixel>>) {
    for y in 0..height {
        for x in 0..width {
            let gray = (image[y][x].get_blue() as u32 + image[y][x].get_green() as u32
            + image[y][x].get_red() as u32) / 3;

            image[y][x].set_blue(gray.min(255) as u8);
            image[y][x].set_green(gray.min(255) as u8);
            image[y][x].set_red(gray.min(255) as u8);
        }
    }
}