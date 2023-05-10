use crate::Buffer;

use super::frame::{self, CRC_FRAME_LENGTH, CRC_MAX_DATA, CRC_RESERVED};

pub struct Rx {
    pub buffer: [u8; CRC_FRAME_LENGTH],
    pub buf_index: usize,
}

impl Rx {
    pub fn update(&mut self, buf: &mut [u8; CRC_MAX_DATA], hw_buf: &mut Buffer) -> Option<usize> {
        let byte_opt = hw_buf.read();

        let outcome = match byte_opt {
            None => None,
            Some(received) => {
                if received == CRC_RESERVED {
                    self.buf_index = 0;
                    self.buffer.fill(0);
                }

                if self.buf_index < CRC_FRAME_LENGTH {
                    self.buffer[self.buf_index] = received;
                    self.buf_index += 1;
                }

                return frame::decode_frame(&self.buffer[0..self.buf_index], buf)
                    .map(|len| {
                        self.buf_index = 0;
                        self.buffer.fill(0);
                        return len;
                    })
                    .map_or(None, |len| Some(len));
            }
        };

        return outcome;
    }

    pub fn new() -> Rx {
        Rx {
            buf_index: 0,
            buffer: [0; CRC_FRAME_LENGTH],
        }
    }
}
