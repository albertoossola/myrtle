use ringbuffer::{
    ConstGenericRingBuffer, RingBuffer, RingBufferExt, RingBufferRead, RingBufferWrite,
};

pub struct Buffer {
    buf: ringbuffer::ConstGenericRingBuffer<u8, 64>,
}

impl Buffer {
    pub fn can_write(&self) -> bool {
        !self.buf.is_full()
    }

    pub fn write(&mut self, data: u8) {
        if self.can_write() {
            self.buf.enqueue(data);
        }
    }

    pub fn peek(&self) -> Option<u8> {
        self.buf.peek().map(|b| *b)
    }

    pub fn read(&mut self) -> Option<u8> {
        self.buf.dequeue()
    }

    pub fn new() -> Buffer {
        Buffer {
            buf: ConstGenericRingBuffer::new(),
        }
    }
}
