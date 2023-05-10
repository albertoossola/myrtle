use crate::crc;

use super::{decode_data_frame, encode_request_frame, FLOW_MAX_DATA};

pub struct Rx {
    seq: u8,
    poll_request: bool,
}

impl Rx {
    pub fn poll_next(&mut self) -> () {
        self.poll_request = true;
    }

    pub fn update(
        &mut self,
        tx: &mut crc::Tx,
        received: Option<&[u8]>,
        buf: &mut [u8; FLOW_MAX_DATA],
    ) -> Option<usize> {
        if tx.is_free() && self.poll_request {
            let mut req_buf = [0; 2];
            encode_request_frame(self.seq, &mut req_buf);

            tx.send(&mut req_buf);
        }

        match received {
            None => None,
            Some(slice) => {
                let received_len = decode_data_frame(slice, self.seq, buf);

                received_len.and_then(|(seq, len)| {
                    self.seq = seq;
                    self.poll_request = false;
                    return Some(len);
                })
            }
        }
    }

    pub fn new() -> Rx {
        Rx {
            poll_request: false,
            seq: 128,
        }
    }
}
