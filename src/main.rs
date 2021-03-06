mod image_processor;
mod screen_buffer;
mod usb;

use crate::screen_buffer::ScreenBuffer;
use crate::usb::Usb;

const BITMAP_SIZE_BYTES: usize = 72;
type RawBitmap = [u8; BITMAP_SIZE_BYTES];

const fn wrap_bitmap(bitmap: &RawBitmap) -> RawBitmap {
    let mut i = 0;

    let mut wrapped_bitmap: RawBitmap = [0; BITMAP_SIZE_BYTES];

    while i < bitmap.len() {
        wrapped_bitmap[i] = bitmap[i].reverse_bits();
        i += 1;
    }

    wrapped_bitmap
}

const NOT_WRAPPED_ARROW_BITMAP: RawBitmap = [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00011111, 0b11110000, 0b00000000, 0b01111000,
    0b00110000, 0b00000000, 0b11011000, 0b00110000, 0b00000001, 0b10011000, 0b00110000, 0b00000011,
    0b00011000, 0b00110000, 0b00000110, 0b00011000, 0b00110000, 0b00001100, 0b00011000, 0b00110000,
    0b00011000, 0b00011000, 0b00110000, 0b00000000, 0b00011000, 0b00000000, 0b00000000, 0b00011000,
    0b00000000, 0b00000000, 0b00011000, 0b00000000, 0b00000000, 0b00011000, 0b00000000, 0b00000000,
    0b00011000, 0b00000000, 0b00000000, 0b00011000, 0b00000000, 0b00000000, 0b00011000, 0b00000000,
    0b00000000, 0b00011000, 0b00000000, 0b00000000, 0b00011000, 0b00000000, 0b00000000, 0b00011000,
    0b00000000, 0b00000000, 0b00011000, 0b00000000, 0b00000000, 0b00011000, 0b00000000, 0b00000000,
    0b00011000, 0b00000000, 0b00000000, 0b00011000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
];

const ARROW_BITMAP: RawBitmap = wrap_bitmap(&NOT_WRAPPED_ARROW_BITMAP);

fn display_bitmap(bitmap: &RawBitmap) {
    let usb = Usb::try_connect();

    let buffer = ScreenBuffer::new(bitmap);

    match usb {
        Ok(usb) => {
            if let Err(error) = usb.send_bytes(&buffer) {
                eprintln!("Error: {}", error);
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}

fn main() {
    display_bitmap(&ARROW_BITMAP);

    let img = image::open("data/test.bmp").unwrap();
    image_processor::print_image(&img);
}
