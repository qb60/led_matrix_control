const SCREEN_SIZE: usize = 24;
pub const SCREEN_BUFFER_SIZE: usize = SCREEN_SIZE / 8 * SCREEN_SIZE;

type ScreenBufferBytes = [u8; SCREEN_BUFFER_SIZE];

pub struct ScreenBuffer {
    pub bytes: ScreenBufferBytes,
}

impl ScreenBuffer {
    pub fn new(bytes: &ScreenBufferBytes) -> Self {
        Self {
            bytes: bytes.clone(),
        }
    }
}
