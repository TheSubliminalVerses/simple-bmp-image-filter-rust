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

pub fn box_blur(width: usize, height: usize, radius: i32, image: &mut Vec<Vec<Pixel>>) {
    let img_cpy = image.clone();

    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let mut red_sum: u32 = 0;
            let mut green_sum: u32 = 0;
            let mut blue_sum: u32 = 0;
            let mut count: u32 = 0;

            for ny in (y as i32 - radius)..(y as i32 + radius) {
                for nx in (x as i32 - radius)..(x as i32 + radius) {
                    if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                        let pixel = img_cpy[ny as usize][nx as usize];

                        red_sum += pixel.get_red() as u32;
                        green_sum += pixel.get_green() as u32;
                        blue_sum += pixel.get_blue() as u32;
                        count += 1;
                    }
                }
            }

            let capped_red: u32 = red_sum.max(red_sum.min(255));
            let capped_green: u32 = green_sum.max(green_sum.min(255));
            let capped_blue: u32 = blue_sum.max(blue_sum.min(255));

            image[y][x].set_blue((capped_blue / count) as u8);
            image[y][x].set_green((capped_green / count) as u8);
            image[y][x].set_red((capped_red / count) as u8);
        }
    }
}

pub fn edge_detection(width: usize, height: usize, radius: i32, image: &mut Vec<Vec<Pixel>>) {
    let img_cpy = image.clone();

    let gx = vec![vec![-1, 0, 1], vec![-2, 0, 2], vec![-1, 0, 1]];
    let gy = vec![vec![1, 2, 1], vec![0, 0, 0], vec![-1, -2, -1]];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut pixel_x_blue = 0;
            let mut pixel_x_green = 0;
            let mut pixel_x_red = 0;

            let mut pixel_y_blue = 0;
            let mut pixel_y_green = 0;
            let mut pixel_y_red = 0;

            for i in -radius..radius {
                for j in -radius..radius {
                    let nx = x as i32 + j;
                    let ny = y as i32 + i;

                    if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                        let pixel = img_cpy[ny as usize][nx as usize];

                        pixel_x_blue += pixel.get_blue() as i32 * gx[(i + 1) as usize][(j + 1) as usize];
                        pixel_x_green += pixel.get_green() as i32 * gx[(i + 1) as usize][(j + 1) as usize];
                        pixel_x_red += pixel.get_red() as i32 * gx[(i + 1) as usize][(j + 1) as usize];

                        pixel_y_blue += pixel.get_blue() as i32 * gy[(i + 1) as usize][(j + 1) as usize];
                        pixel_y_green += pixel.get_green() as i32 * gy[(i + 1) as usize][(j + 1) as usize];
                        pixel_y_red += pixel.get_red() as i32 * gy[(i + 1) as usize][(j + 1) as usize];
                    }
                }
            }

            let mag_blue = ((pixel_x_blue * pixel_x_blue) + (pixel_y_blue * pixel_y_blue)).isqrt();
            let mag_green = ((pixel_x_green * pixel_x_green) + (pixel_y_green * pixel_y_green)).isqrt();
            let mag_red = ((pixel_x_red * pixel_x_red) + (pixel_y_red * pixel_y_red)).isqrt();

            let blue = mag_blue.max(mag_blue.min(255));
            let green = mag_green.max(mag_green.min(255));
            let red = mag_red.max(mag_red.min(255));

            image[y][x].set_blue(blue as u8);
            image[y][x].set_green(green as u8);
            image[y][x].set_red(red as u8);
        }
    }
}