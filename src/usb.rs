use crate::screen_buffer;
use crate::screen_buffer::ScreenBuffer;
use hidapi::{HidApi, HidDevice, HidError};
use thiserror::Error;

pub struct Usb {
    device: HidDevice,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error in HIDAPI: {:?}", .0)]
    HidApiError(#[from] HidError),
    #[error("Device not found")]
    DeviceNotFound,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
const PACKET_SIZE: usize = screen_buffer::SCREEN_BUFFER_SIZE;

impl Usb {
    pub fn try_connect() -> Result<Self> {
        const PRODUCT_STRING: &str = "USB LED MATRIX";

        let hid_api = HidApi::new()?;
        let device_info = hid_api
            .device_list()
            .find(|info| info.product_string() == Some(PRODUCT_STRING));

        match device_info {
            None => Err(Error::DeviceNotFound),
            Some(device_info) => {
                let device = device_info.open_device(&hid_api)?;
                Ok(Self { device })
            }
        }
    }

    pub fn send_bytes(self, buffer: &ScreenBuffer) -> Result<()> {
        let mut packet: [u8; PACKET_SIZE + 1] = [0; PACKET_SIZE + 1];
        packet[1..].copy_from_slice(&buffer.bytes);
        self.device.send_feature_report(&packet)?;
        Ok(())
    }
}
