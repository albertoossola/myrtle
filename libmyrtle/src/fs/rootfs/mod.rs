use crate::fs::FileSystem;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};

use super::path::Path;
use super::FsError;

pub struct RootFs {
    mount_points: BTreeMap<String, Box<dyn FileSystem>>,
}

impl RootFs {
    fn new() -> RootFs {
        return RootFs {
            mount_points: BTreeMap::new(),
        };
    }

    pub fn mount(&mut self, mount: &str, fs: Box<dyn FileSystem>) -> Result<(), FsError> {
        self.mount_points.insert(mount.to_string(), fs);

        Ok(())
    }
}

impl FileSystem for RootFs {
    fn run(&mut self, path: &Path, command: &mut super::FsCommand) -> Result<(), FsError> {
        match path {
            Path::Segment(mount_point, subpath) => {
                let mut fs = self
                    .mount_points
                    .get_mut(*mount_point)
                    .ok_or(FsError::NotFound)?;

                return fs.run(subpath, command);
            }
            Path::End => Err(FsError::InvalidPath),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::fs::rootfs::RootFs;

    #[test]
    pub fn invalid_mount_path() {
        let mut rootfs = RootFs::new();
        //let mock_fs = MockFileSystem::new();

        //_mod.mount("foo/bar", mock_fs);
    }
}
