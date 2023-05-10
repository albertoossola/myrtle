pub const CRC_RESERVED: u8 = 0x61;

pub const CRC_MAX_DATA: usize = 34;
pub const CRC_HEADER_SIZE: usize = 3;
pub const CRC_OVERHEAD: usize = CRC_HEADER_SIZE + 1;
pub const CRC_FRAME_LENGTH: usize = CRC_MAX_DATA + CRC_OVERHEAD;

pub enum FrameError {
    InvalidChecksum,
    InvalidLength,
    InvalidHeader,
}

pub fn encode_frame(data: &[u8], buf: &mut [u8; CRC_FRAME_LENGTH]) -> bool {
    let len = data.len();

    if len > CRC_MAX_DATA {
        return false;
    }

    buf[0] = CRC_RESERVED;
    buf[1] = len as u8;
    buf[2] = 0x00;

    for (i, b) in data.iter().enumerate() {
        buf[i + CRC_HEADER_SIZE] = *b;
    }

    let mut last_reserved = 2;
    let mut checksum: u8 = 0x00;

    //Replace all the occurrences of 0xCC with a linked list
    for i in CRC_HEADER_SIZE..(len + CRC_HEADER_SIZE) {
        if buf[i] == CRC_RESERVED {
            buf[last_reserved] = i as u8;
            last_reserved = i as usize;
        }

        checksum = (checksum.wrapping_mul(3) ^ buf[i]) & 0xFF;
    }

    buf[last_reserved] = 0x00;

    buf[3 + len] = checksum;

    return true;
}

pub fn decode_frame(data: &[u8], buf: &mut [u8; CRC_MAX_DATA]) -> Result<usize, FrameError> {
    if data.len() < CRC_OVERHEAD {
        return Err(FrameError::InvalidLength);
    }

    if data[0] != CRC_RESERVED {
        return Err(FrameError::InvalidHeader);
    }

    let len: usize = data[1] as usize;

    if len > (data.len() - CRC_OVERHEAD) {
        return Err(FrameError::InvalidLength);
    }

    let mut next_reserved: usize = data[2] as usize;
    let mut buf_cursor = 0;
    let mut checksum: u8 = 0x00;

    for i in 3..(3 + len) {
        buf[buf_cursor] = data[i];

        if i == next_reserved {
            next_reserved = data[i] as usize;
            buf[buf_cursor] = CRC_RESERVED;
        }

        checksum = checksum.wrapping_mul(3) ^ buf[buf_cursor];
        buf_cursor += 1;
    }

    if checksum != data[3 + len] {
        return Err(FrameError::InvalidChecksum);
    }

    return Ok(len);
}
