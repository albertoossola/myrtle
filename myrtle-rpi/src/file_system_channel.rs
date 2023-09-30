use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use libmyrtle::interface::channels::Channel;

pub struct FileSystemChannel {
    max_size : usize,
    cursor : usize,
    file : File
}

impl Channel for FileSystemChannel {
    fn has_room_for(&self, buffer: &[u8]) -> bool {
        return self.cursor + (buffer.len()) < self.max_size;
    }

    fn write(&mut self, buffer: &[u8]) -> () {
        if self.has_room_for(buffer) {
            self.file.write_all(buffer).ok();
        }

        self.cursor += buffer.len();
    }

    fn rewind(&mut self) -> () {
        self.file.seek(SeekFrom::Start(0)).ok();
        self.cursor = 0;
    }

    fn get_string_or_none(&mut self) -> Option<String> {
        let mut string = String::new();

        return self.file
            .read_to_string(&mut string)
            .ok()
            .map(|_| string);
    }
}

impl FileSystemChannel {
    pub fn new(path : &str, max_size : usize) -> FileSystemChannel {
        let file = File::options()
            .create(true)
            .write(true)
            .open(path)
            .unwrap();

        return FileSystemChannel {
            cursor: 0,
            max_size,
            file
        }
    }
}

