use alloc::string::String;

pub trait Channel {
    fn has_room_for(&self, buffer: &[u8]) -> bool;
    fn write(&mut self, buffer: &[u8]) -> ();
    fn rewind(&mut self) -> ();
    fn get_string_or_none(&mut self) -> Option<String>;
}
