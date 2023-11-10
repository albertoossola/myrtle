use super::ShellError;

pub trait ShellIO {
    fn write(&mut self, c : u8) -> Result<(), ShellError>;
    fn read(&mut self) -> Option<u8>;
}