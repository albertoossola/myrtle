#[derive(Debug, Clone)]
pub enum ErrorCode {
    OutOfMemory,
    InvalidSyntax,
    ArgumentRequired,
    InvalidArgumentType,
    UnknownNodeKind,
    EntryStateRequired
}
