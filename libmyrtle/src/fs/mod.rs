mod path;
mod rootfs;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::result::Iter;

use mockall::automock;

pub enum FsError {
    NoRoomLeft,
    NotFound,
    InvalidPath,
    AlreadyPresent,
    InvalidMountPoint
}

#[automock]
pub trait FileSystem {
    fn get_directories_at(&self, path : &str, item_callback : fn (name : &str) -> ()) -> Result<(), FsError>;
    fn get_files_at(&self, path : &str, item_callback : fn (name : &str) -> ()) -> Result<(), FsError>;
    fn make_file(&mut self, path: &str) -> Result<(), FsError>;
    fn make_directory(&mut self, path: &str) -> Result<(), FsError>;
    fn delete(&mut self, path: &str) -> Result<(), FsError>;
    fn truncate(&mut self, path: &str) -> Result<(), FsError>;
    fn append(&mut self, path: &str, content: &[u8]) -> Result<(), FsError>;
}