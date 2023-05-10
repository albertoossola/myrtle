use crate::Buffer;

use super::frame::{encode_frame, CRC_FRAME_LENGTH, CRC_MAX_DATA};

pub struct Tx {
    is_sending: bool,
    buffer: [u8; CRC_FRAME_LENGTH],
    buffer_index: usize,
}

impl Tx {
    pub fn is_free(&self) -> bool {
        return !self.is_sending;
    }

    pub fn send(&mut self, frame: &[u8]) {
        if frame.len() > CRC_MAX_DATA {
            return;
        }

        if !self.is_free() {
            return;
        }

        if !encode_frame(frame, &mut self.buffer) {
            return;
        }

        self.buffer_index = 0;
        self.is_sending = true;
    }

    pub fn update(&mut self, hw_buf: &mut Buffer) -> () {
        if !self.is_sending {
            return;
        }

        if !hw_buf.can_write() {
            return;
        }

        let next_byte = self.buffer[self.buffer_index];
        self.buffer_index += 1;

        hw_buf.write(next_byte);

        if self.buffer_index == CRC_FRAME_LENGTH {
            self.is_sending = false;
        }
    }

    pub fn new() -> Tx {
        Tx {
            is_sending: false,
            buffer: [0; CRC_FRAME_LENGTH],
            buffer_index: 0,
        }
    }
}
