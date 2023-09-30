use alloc::string::String;
use super::Channel;

const MEMORY_BUFFER_LENGTH: usize = 1024 * 2;

pub struct MemoryBufferChannel {
    buffer: [u8; MEMORY_BUFFER_LENGTH],
    index: usize,
}

impl Channel for MemoryBufferChannel {
    fn has_room_for(&self, data: &[u8]) -> bool {
        let bytes_left = self.buffer.len() - self.index;
        return bytes_left >= data.len();
    }

    fn write(&mut self, data: &[u8]) -> () {
        if self.has_room_for(data) {
            self.buffer[self.index..].copy_from_slice(data);
            self.index += data.len();
        }
    }

    fn rewind(&mut self) -> () {
        self.index = 0;
    }

    fn get_string_or_none(&mut self) -> Option<String> {
        return match core::str::from_utf8(&self.buffer[..self.index]) {
            Ok(string) => Some(String::from(string)),
            _ => None,
        };
    }
}

impl MemoryBufferChannel {
    pub fn new() -> MemoryBufferChannel {
        return MemoryBufferChannel {
            buffer: [0; MEMORY_BUFFER_LENGTH],
            index: 0,
        };
    }
}
