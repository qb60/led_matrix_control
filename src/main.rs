use crate::usb::{Usb, Error};
use std::convert::TryInto;

mod usb;

fn main() {
    let usb = Usb::try_connect();

    match usb {
        Ok(usb) => {
            let mut bytes: [u8; 72] = [0; 72];
            for i in 1..72 {
                bytes[i-1] = i.try_into().unwrap();
            }

            usb.send_bytes(&bytes);

        },
        Err(error) => {
            eprintln!("Error: {:?}", error);
        },
    }
}