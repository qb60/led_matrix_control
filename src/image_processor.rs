use image::{DynamicImage, GenericImageView, Pixel};

pub fn print_image(image: &DynamicImage) {
    let (width, height) = image.dimensions();

    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let luma = pixel.to_luma();
            let brightness = luma.0[0];

            let char = if brightness < 255 { "0" } else { "_" };

            line += char;
        }
        println!("{}", line);
    }
}
