pub const DATA_ACK: u8 = 0x01;
pub const DATA_FRAME: u8 = 0x02;

pub const FLOW_MAX_DATA: usize = 32;
pub const FLOW_OVERHEAD: usize = 2;
pub const FLOW_FRAME_LEN: usize = FLOW_MAX_DATA + FLOW_OVERHEAD;

pub fn encode_request_frame(seq: u8, buf: &mut [u8; 2]) -> () {
    buf[0] = DATA_ACK;
    buf[1] = seq;
}

pub fn encode_data_frame(seq: u8, data: &[u8], buf: &mut [u8; FLOW_FRAME_LEN]) -> Option<usize> {
    if data.len() > FLOW_MAX_DATA {
        return None;
    }

    buf[0] = DATA_FRAME;
    buf[1] = seq;
    buf[FLOW_OVERHEAD..(FLOW_OVERHEAD + data.len())].copy_from_slice(data);

    return Some(data.len() + FLOW_OVERHEAD);
}

pub fn decode_data_frame(
    frame: &[u8],
    seq: u8,
    buf: &mut [u8; FLOW_MAX_DATA],
) -> Option<(u8, usize)> {
    if !(FLOW_OVERHEAD..=FLOW_FRAME_LEN).contains(&frame.len()) {
        return None;
    }

    if frame[0] != DATA_FRAME {
        return None;
    }

    //In case the sequence number is the same as now, return None;
    if frame[1] == seq {
        return None;
    }

    buf[0..frame.len() - FLOW_OVERHEAD].copy_from_slice(&frame[(FLOW_OVERHEAD)..]);

    return Some((frame[1], frame.len() - FLOW_OVERHEAD));
}

pub fn decode_request_frame(frame: &[u8]) -> Option<u8> {
    match frame {
        [DATA_ACK, seq, ..] => Some(*seq),
        _ => None,
    }
}
