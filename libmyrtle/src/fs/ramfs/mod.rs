use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::path::Path;
use super::{FileSystem, FsCommand, FsError};

enum RamFsNode {
    Dir(BTreeMap<String, RamFsNode>),
    File(Vec<u8>),
}

struct RamFs {}

impl RamFs {
    fn run_on_node(&mut self, node: RamFsNode, path: &Path) {}
}

struct RamFsDirectory {
    directories: BTreeMap<String, RamFsDirectory>,
    files: BTreeMap<String, RamFsFile>,
}

impl RamFsDirectory {
    pub fn new() -> RamFsDirectory {
        RamFsDirectory {
            directories: BTreeMap::new(),
            files: BTreeMap::new(),
        }
    }

    pub fn send_command_to_child(
        &mut self,
        name: &str,
        subpath: &Path,
        command: FsCommand,
    ) -> Result<(), super::FsError> {
        return match self.directories.get_mut(name) {
            Some(dir) => dir.run(subpath, command),
            None => match self.files.get_mut(name) {
                Some(dir) => dir.run(subpath, command),
                None => Err(FsError::InvalidPath),
            },
        };
    }
}

impl FileSystem for RamFsDirectory {
    fn run(
        &mut self,
        path: &super::path::Path,
        command: super::FsCommand,
    ) -> Result<(), super::FsError> {
        match path {
            Path::End => match command {
                super::FsCommand::GetDirs(callback) => {
                    self.directories.keys().for_each(|k| callback(k));
                    Ok(())
                }
                super::FsCommand::GetFiles(callback) => {
                    self.files.keys().for_each(|k| callback(k));
                    Ok(())
                }
                super::FsCommand::MakeDir(dir_name) => {
                    self.directories
                        .insert(dir_name.to_string(), RamFsDirectory::new());
                    Ok(())
                }
                super::FsCommand::MakeFile(file_name) => {
                    self.files.insert(file_name.to_string(), RamFsFile::new());
                    Ok(())
                }
                _ => Err(FsError::InvalidPath),
            },
            Path::Segment(segment, subpath) => {
                self.send_command_to_child(&segment, &subpath, command)
            }
        }
    }
}

struct RamFsFile {
    buffer: Vec<u8>,
}

impl FileSystem for RamFsFile {
    fn run(&mut self, path: &Path, command: super::FsCommand) -> Result<(), FsError> {
        match path {
            Path::End => match command {
                FsCommand::AppendToFile(content) => {
                    self.buffer.extend_from_slice(content);
                    Ok(())
                }
                FsCommand::TruncateFile() => {
                    self.buffer.clear();
                    Ok(())
                }
                FsCommand::ReadFile(offset, lenght, callback) => {
                    callback(&self.buffer[offset..lenght]);
                    Ok(())
                }
                _ => Err(FsError::InvalidPath),
            },
            _ => Err(FsError::InvalidPath),
        }
    }
}

impl RamFsFile {
    pub fn new() -> RamFsFile {
        RamFsFile { buffer: Vec::new() }
    }
}
