use crate::{
    crc::{self, CRC_MAX_DATA},
    flow::{self, FLOW_MAX_DATA},
    Buffer,
};

pub const MAX_DATA: usize = FLOW_MAX_DATA;

pub struct RTx {
    pub crc_tx: crc::Tx,
    pub crc_rx: crc::Rx,

    pub flow_tx: flow::Tx,
    pub flow_rx: flow::Rx,
}

pub struct RTxContext<'a> {
    pub send_byte: &'a mut dyn FnMut(u8) -> (),
    pub can_send: &'a dyn Fn() -> bool,
    pub read_byte: &'a mut dyn FnMut() -> Option<u8>,
}

impl RTx {
    pub fn is_free(&self) -> bool {
        return self.flow_tx.is_free();
    }

    pub fn send(&mut self, data: &[u8]) {
        self.flow_tx.send(data);
    }

    pub fn poll_next(&mut self) -> () {
        self.flow_rx.poll_next();
    }

    pub fn update(
        &mut self,
        tx_buf: &mut Buffer,
        rx_buf: &mut Buffer,
        received_frame: &mut [u8; FLOW_MAX_DATA],
    ) -> Option<usize> {
        //Update the low level Tx and Rx
        let mut crc_rx_buf = [0; CRC_MAX_DATA];
        let crc_rx_opt = self.crc_rx.update(&mut crc_rx_buf, rx_buf);

        self.crc_tx.update(tx_buf);

        //Update the flow-controlled Rx and Tx
        self.flow_tx
            .update(&mut self.crc_tx, crc_rx_opt.map(|l| &crc_rx_buf[..l]));

        return self.flow_rx.update(
            &mut self.crc_tx,
            crc_rx_opt.map(|l| &crc_rx_buf[..l]),
            received_frame,
        );
    }

    pub fn new() -> RTx {
        RTx {
            crc_tx: crc::Tx::new(),
            crc_rx: crc::Rx::new(),
            flow_tx: flow::Tx::new(),
            flow_rx: flow::Rx::new(),
        }
    }
}
