pub enum Command {
    Run,
    SetChannel(u8),
    Write(u8),
}

pub trait CommandSource {
    fn poll(&mut self) -> Option<Command>;
}
