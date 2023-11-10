use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::path::Path;
use super::{FileSystem, FsCommand, FsError};

enum RamFsNode {
    Dir(BTreeMap<String, RamFsNode>),
    File(Vec<u8>),
}

pub struct RamFs {
    root: RamFsNode,
}

impl RamFs {
    fn run_on_node(
        node: &mut RamFsNode,
        path: &Path,
        command: &mut FsCommand,
    ) -> Result<(), FsError> {
        return match (node, path) {
            (RamFsNode::Dir(subnodes), Path::End) => match command {
                FsCommand::GetDirs(callback) => {
                    subnodes
                        .iter()
                        .filter(|n| match n {
                            (_, RamFsNode::Dir(_)) => true,
                            _ => false,
                        })
                        .for_each(|(k, _)| callback(k));

                    Ok(())
                }
                FsCommand::GetFiles(callback) => {
                    subnodes
                        .iter()
                        .filter(|n| match n {
                            (_, RamFsNode::File(_)) => true,
                            _ => false,
                        })
                        .for_each(|(k, _)| callback(k));

                    Ok(())
                }
                _ => Err(FsError::InvalidPath),
            },
            (RamFsNode::Dir(subnodes), Path::Segment(child_name, subpath)) => {
                match (command, subpath.as_ref()) {
                    (FsCommand::Delete, Path::End) => {
                        subnodes.remove(*child_name);
                        Ok(())
                    }

                    (FsCommand::MakeDir, Path::End) => {
                        if subnodes.contains_key(*child_name) {
                            return Err(FsError::AlreadyPresent);
                        }

                        subnodes.insert(child_name.to_string(), RamFsNode::Dir(BTreeMap::new()));

                        Ok(())
                    }
                    (FsCommand::MakeFile, Path::End) => {
                        if subnodes.contains_key(*child_name) {
                            return Err(FsError::AlreadyPresent);
                        }

                        subnodes.insert(child_name.to_string(), RamFsNode::File(Vec::new()));

                        Ok(())
                    }
                    (command, _) => {
                        let mut subnode = subnodes.get_mut(*child_name).ok_or(FsError::NotFound)?;
                        return Self::run_on_node(subnode, subpath, command);
                    }
                }
            }
            (RamFsNode::File(file_buffer), Path::End) => match command {
                FsCommand::AppendToFile(content) => {
                    file_buffer.extend_from_slice(content);
                    Ok(())
                }
                FsCommand::TruncateFile => {
                    file_buffer.clear();
                    Ok(())
                }
                FsCommand::ReadFile(offset, length, callback) => {
                    let clamped_offset = (*offset).clamp(0, file_buffer.len());
                    let clamped_length = (*length).clamp(0, file_buffer.len() - clamped_offset);

                    let slice = &file_buffer[clamped_offset..clamped_length];
                    callback(slice);
                    Ok(())
                }
                _ => Err(FsError::FileExpected),
            },
            _ => Err(FsError::InvalidPath),
        };
    }

    pub fn new() -> Self {
        RamFs {
            root: RamFsNode::Dir(BTreeMap::new()),
        }
    }
}

impl FileSystem for RamFs {
    fn run(&mut self, path: &Path, command: &mut FsCommand) -> Result<(), FsError> {
        Self::run_on_node(&mut self.root, path, command)
    }
}

#[cfg(test)]
mod test {
    use crate::fs::{path::Path, FileSystem, FsCommand};

    use super::RamFs;

    #[test]
    pub fn get_files_empty() {
        let mut ramfs = RamFs::new();
        let mut files_count = 0;

        ramfs
            .run(
                &Path::End,
                &mut FsCommand::GetFiles(&mut |_| {
                    files_count += 1;
                }),
            )
            .unwrap();

        assert_eq!(files_count, 0);
    }

    #[test]
    pub fn get_dirs_empty() {
        let mut ramfs = RamFs::new();
        let mut dirs_count = 0;

        ramfs
            .run(
                &Path::End,
                &mut &mut FsCommand::GetDirs(&mut |_| {
                    dirs_count += 1;
                }),
            )
            .unwrap();

        assert_eq!(dirs_count, 0);
    }

    #[test]
    pub fn create_file() {
        let mut ramfs = RamFs::new();
        let mut files_count = 0;

        let path = Path::new("foo").unwrap();

        ramfs.run(&path, &mut FsCommand::MakeFile).unwrap();
        ramfs
            .run(
                &Path::End,
                &mut FsCommand::GetFiles(&mut |_| {
                    files_count += 1;
                }),
            )
            .unwrap();

        assert_eq!(files_count, 1);
    }

    #[test]
    pub fn create_dir() {
        let mut ramfs = RamFs::new();
        let mut dirs_count = 0;

        let path = Path::new("foo").unwrap();

        ramfs.run(&path, &mut FsCommand::MakeDir).unwrap();
        ramfs
            .run(
                &Path::End,
                &mut FsCommand::GetDirs(&mut |_| {
                    dirs_count += 1;
                }),
            )
            .unwrap();

        assert_eq!(dirs_count, 1);
    }

    #[test]
    pub fn delete_file() {
        let mut ramfs = RamFs::new();
        let mut files_count = 0;

        let path = Path::new("foo").unwrap();

        ramfs.run(&path, &mut FsCommand::MakeFile).unwrap();
        ramfs
            .run(&Path::new("foo").unwrap(), &mut FsCommand::Delete)
            .unwrap();
        ramfs
            .run(
                &Path::End,
                &mut FsCommand::GetFiles(&mut |_| {
                    files_count += 1;
                }),
            )
            .unwrap();

        assert_eq!(files_count, 0);
    }

    #[test]
    pub fn delete_dir() {
        let mut ramfs = RamFs::new();
        let mut dirs_count = 0;

        let path = Path::new("foo").unwrap();

        ramfs.run(&path, &mut FsCommand::MakeDir).unwrap();
        ramfs
            .run(&Path::new("foo").unwrap(), &mut FsCommand::Delete)
            .unwrap();
        ramfs
            .run(
                &Path::End,
                &mut FsCommand::GetDirs(&mut |_| {
                    dirs_count += 1;
                }),
            )
            .unwrap();

        assert_eq!(dirs_count, 0);
    }

    #[test]
    pub fn create_nested_file() {
        let mut ramfs = RamFs::new();
        let mut files_count = 0;

        let dir_path = Path::new("foo").unwrap();

        ramfs.run(&dir_path, &mut FsCommand::MakeDir).unwrap();

        let file_path = Path::new("foo/bar").unwrap();

        ramfs.run(&file_path, &mut FsCommand::MakeFile).unwrap();
        ramfs
            .run(
                &Path::new("foo").unwrap(),
                &mut FsCommand::GetFiles(&mut |_| {
                    files_count += 1;
                }),
            )
            .unwrap();

        assert_eq!(files_count, 1);
    }

    #[test]
    pub fn write_and_read_file() {
        let mut ramfs = RamFs::new();
        let mut write_and_read_correct = false;

        let path = Path::new("foo").unwrap();

        ramfs.run(&path, &mut FsCommand::MakeFile).unwrap();
        ramfs
            .run(&path, &mut &mut FsCommand::AppendToFile(&[1, 2, 3]))
            .unwrap();
        ramfs
            .run(
                &path,
                &mut FsCommand::ReadFile(0, 10, &mut |data| match data {
                    [1, 2, 3] => write_and_read_correct = true,
                    _ => write_and_read_correct = false,
                }),
            )
            .unwrap();

        assert!(write_and_read_correct);
    }
}
