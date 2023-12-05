use self::path::Path;

pub mod path;
pub mod ramfs;
pub mod rootfs;

#[derive(Debug)]
pub enum FsError {
    NoRoomLeft,
    NotFound,
    InvalidPath,
    AlreadyPresent,
    InvalidMountPoint,
    OutOfBounds,
    DirectoryExpected,
    FileExpected,
}

pub enum FsCommand<'a> {
    GetFiles(&'a mut dyn FnMut(&str) -> ()),
    GetDirs(&'a mut dyn FnMut(&str) -> ()),
    MakeFile,
    MakeDir,
    Delete,
    TruncateFile,
    AppendToFile(&'a [u8]),
    ReadFile(usize, usize, &'a mut dyn FnMut(&[u8]) -> ()),
}

pub trait FileSystem {
    fn run(&mut self, path: &Path, command: &mut FsCommand) -> Result<(), FsError>;
}
