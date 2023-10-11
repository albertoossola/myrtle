use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use core::cell::RefCell;
use crate::fs::{FileSystem, FsError};
use crate::fs::path::Path;

pub struct RootFs {
    mount_points: BTreeMap<String, Rc<RefCell<Box<dyn FileSystem>>>>
}

impl RootFs {
    fn new() -> RootFs {
        return RootFs {
          mount_points: BTreeMap::new()
        };
    }

    fn take_fs(&self, mount_point : &str) -> Result<Rc<RefCell<Box<dyn FileSystem>>>, FsError> {
        return self.mount_points
            .get(mount_point)
            .map(|mp| mp.clone())
            .ok_or(FsError::InvalidMountPoint);
    }

    fn take_fs_mut(&mut self, mount_point : &str) -> Result<Rc<RefCell<Box<dyn FileSystem>>>, FsError> {
        return self.mount_points
            .get(mount_point)
            .map(|mp| mp.clone())
            .ok_or(FsError::InvalidMountPoint);
    }

    pub fn mount(&mut self, path: &str, fs: Box<dyn FileSystem>) -> Result<(), FsError> {
        let (head, tail) = Path::get_head_tail(path).ok_or(FsError::InvalidMountPoint)?;

        if !tail.is_empty() {
            return Err(FsError::InvalidMountPoint);
        }

        self.mount_points.insert(head.to_string(), Rc::new(RefCell::new(fs)));

        return Ok(());
    }
}

impl FileSystem for RootFs {
    fn get_directories_at(&self, path: &str, callback: fn (name : &str) -> ()) -> Result<(), FsError> {
        let (mount_point, local_path) = Path::get_head_tail(path).ok_or(FsError::InvalidPath)?;
        let fs_ref = self.take_fs(mount_point)?;
        let fs = fs_ref.borrow();

        return fs.get_directories_at(local_path, callback);
    }

    fn get_files_at(&self, path: &str, callback: fn (name : &str) -> ()) -> Result<(), FsError> {
        let (mount_point, local_path) = Path::get_head_tail(path).ok_or(FsError::InvalidPath)?;
        let fs_ref = self.take_fs(mount_point)?;
        let fs = fs_ref.borrow();

        return fs.get_files_at(local_path, callback);
    }

    fn make_file(&mut self, path: &str) -> Result<(), FsError> {
        let (mount_point, local_path) = Path::get_head_tail(path).ok_or(FsError::InvalidPath)?;
        let mut fs_ref = self.take_fs(mount_point)?;
        let mut fs = fs_ref.borrow_mut();

        return fs.make_file(local_path);
    }

    fn make_directory(&mut self, path: &str) -> Result<(), FsError> {
        let (mount_point, local_path) = Path::get_head_tail(path).ok_or(FsError::InvalidPath)?;
        let mut fs_ref = self.take_fs(mount_point)?;
        let mut fs = fs_ref.borrow_mut();

        return fs.make_directory(local_path);
    }

    fn delete(&mut self, path: &str) -> Result<(), FsError> {
        let (mount_point, local_path) = Path::get_head_tail(path).ok_or(FsError::InvalidPath)?;
        let mut fs_ref = self.take_fs(mount_point)?;
        let mut fs = fs_ref.borrow_mut();

        return fs.delete(local_path);
    }

    fn truncate(&mut self, path: &str) -> Result<(), FsError> {
        let (mount_point, local_path) = Path::get_head_tail(path).ok_or(FsError::InvalidPath)?;
        let mut fs_ref = self.take_fs(mount_point)?;
        let mut fs = fs_ref.borrow_mut();

        return fs.truncate(local_path);
    }

    fn append(&mut self, path: &str, content: &[u8]) -> Result<(), FsError> {
        let (mount_point, local_path) = Path::get_head_tail(path).ok_or(FsError::InvalidPath)?;
        let mut fs_ref = self.take_fs(mount_point)?;
        let mut fs = fs_ref.borrow_mut();

        return fs.append(local_path, content);
    }
}

#[cfg(test)]
mod test {
    use mockall::mock;
    use crate::fs::FileSystem;
    use crate::fs::rootfs::RootFs;

    #[test]
    pub fn invalid_mount_path() {
        let mut rootfs = RootFs::new();
        //let mock_fs = MockFileSystem::new();

        //_mod.mount("foo/bar", mock_fs);
    }
}