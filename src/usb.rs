use hidapi::{HidApi, HidDevice, HidError, DeviceInfo};
use thiserror::Error;
use crate::usb::Error::DeviceNotFound;

const SCREEN_SIZE: usize = 24;
const PACKET_SIZE: usize = SCREEN_SIZE/8*SCREEN_SIZE;

pub struct Usb {
    device: HidDevice
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error in HIDAPI: {:?}", .0)]
    HidApiError(#[from] HidError),
    #[error("Device not found")]
    DeviceNotFound,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;


impl Usb {
    pub fn try_connect() -> Result<Self> {
        const PRODUCT_STRING: &str = "USB LED MATRIX";

        let hid_api = HidApi::new()?;
        let device_info = hid_api.device_list().find(|info| info.product_string() == Some(PRODUCT_STRING));

        return match device_info {
            None => {
                Err(DeviceNotFound)
            },
            Some(device_info) => {
                let device = device_info.open_device(&hid_api)?;
                Ok(Self { device })
            },
        }
    }

    pub fn send_bytes(self, bytes: &[u8; PACKET_SIZE]) -> Result<()>{
        let mut packet: [u8; PACKET_SIZE+1] = [0; PACKET_SIZE+1];
        packet[1..].copy_from_slice(bytes);
        self.device.send_feature_report(&packet)?;
        Ok(())
    }
}