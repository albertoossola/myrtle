use crate::crc;

use super::{decode_request_frame, encode_data_frame, FLOW_FRAME_LEN, FLOW_MAX_DATA};

pub struct Tx {
    pub buffer: [u8; FLOW_MAX_DATA],
    pub buffer_len: usize,
    pub seq: u8,
    pub new_data_required: bool,
}

impl Tx {
    pub fn is_free(&self) -> bool {
        return self.new_data_required;
    }

    pub fn send(&mut self, data: &[u8]) {
        if !self.is_free() {
            return;
        }

        self.new_data_required = false;
        self.buffer_len = data.len();
        self.buffer[..self.buffer_len].copy_from_slice(data);
    }

    pub fn update(&mut self, tx: &mut crc::Tx, received: Option<&[u8]>) {
        if !self.new_data_required && tx.is_free() {
            let mut data_buf: [u8; FLOW_FRAME_LEN] = [0; FLOW_FRAME_LEN];

            match encode_data_frame(self.seq, &self.buffer[..self.buffer_len], &mut data_buf) {
                Some(len) => tx.send(&data_buf[..len]),
                None => {}
            }
        }

        match received {
            None => {}
            Some(slice) => {
                decode_request_frame(slice).and_then(|seq| {
                    if self.seq == seq {
                        self.new_data_required = true;
                        self.seq = self.seq.wrapping_add(1);
                    }

                    Some(seq)
                });
            }
        };
    }

    pub fn new() -> Tx {
        Tx {
            new_data_required: true,
            seq: 0,
            buffer_len: 0,
            buffer: [0; FLOW_MAX_DATA],
        }
    }
}
