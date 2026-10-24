//! Descriptor-relative writes keep bootstrap destinations behind symlink checks.
use anyhow::{Context, Result, ensure};
use std::{
    ffi::CString,
    fs::File,
    io::{Read, Seek, Write},
    os::{
        fd::{AsRawFd, FromRawFd},
        unix::fs::MetadataExt,
    },
    path::{Component, Path},
};

pub struct Root(File);

fn open(parent: &File, name: &str, flags: i32, mode: u32) -> std::io::Result<File> {
    let name = CString::new(name)?;
    let fd = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            name.as_ptr(),
            flags | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            mode,
        )
    };
    if fd < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

impl Root {
    pub fn new(path: &Path) -> Result<Self> {
        Ok(Self(File::open(path)?))
    }

    fn parent(&self, path: &str, create: bool) -> Result<Option<(File, String)>> {
        let mut parts = Vec::new();
        for part in Path::new(path).components() {
            match part {
                Component::Normal(s) => parts.push(s.to_str().context("non-UTF8 destination")?),
                _ => anyhow::bail!(
                    "destination must be repository-relative without traversal: {path}"
                ),
            }
        }
        let leaf = parts.pop().context("empty destination")?.to_string();
        let mut dir = self.0.try_clone()?;
        for name in parts {
            match open(&dir, name, libc::O_RDONLY | libc::O_DIRECTORY, 0) {
                Ok(child) => dir = child,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound && create => {
                    let name_c = CString::new(name)?;
                    let result = unsafe { libc::mkdirat(dir.as_raw_fd(), name_c.as_ptr(), 0o755) };
                    if result != 0 {
                        let error = std::io::Error::last_os_error();
                        ensure!(
                            error.kind() == std::io::ErrorKind::AlreadyExists,
                            "create directory {name}: {error}"
                        );
                    }
                    dir.sync_all()?;
                    dir = open(&dir, name, libc::O_RDONLY | libc::O_DIRECTORY, 0)?;
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
                Err(e) => {
                    return Err(e).with_context(|| format!("unsafe destination directory {path}"));
                }
            }
        }
        Ok(Some((dir, leaf)))
    }

    pub fn read(&self, path: &str) -> Result<Option<Vec<u8>>> {
        let Some((parent, leaf)) = self.parent(path, false)? else {
            return Ok(None);
        };
        let mut file = match open(&parent, &leaf, libc::O_RDONLY | libc::O_NONBLOCK, 0) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e).with_context(|| format!("unsafe destination {path}")),
        };
        let metadata = file.metadata()?;
        ensure!(
            metadata.is_file() && metadata.nlink() == 1 && metadata.len() <= 4 * 1024 * 1024,
            "destination must be a single regular file below 4 MiB: {path}"
        );
        let mut bytes = Vec::new();
        (&mut file)
            .take(4 * 1024 * 1024 + 1)
            .read_to_end(&mut bytes)?;
        ensure!(
            bytes.len() <= 4 * 1024 * 1024,
            "destination grew beyond limit: {path}"
        );
        Ok(Some(bytes))
    }

    pub fn directory(&self, path: &str) -> Result<bool> {
        let Some((parent, leaf)) = self.parent(path, false)? else {
            return Ok(false);
        };
        match open(&parent, &leaf, libc::O_RDONLY | libc::O_DIRECTORY, 0) {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e).with_context(|| format!("unsafe directory {path}")),
        }
    }

    pub fn create(&self, path: &str, bytes: &[u8]) -> Result<()> {
        let (parent, leaf) = self
            .parent(path, true)?
            .context("missing destination parent")?;
        let temporary = format!(".allstack-init-{}", uuid::Uuid::new_v4());
        let temporary_c = CString::new(temporary.as_str())?;
        let leaf_c = CString::new(leaf)?;
        let result: Result<()> = (|| {
            let mut file = open(
                &parent,
                &temporary,
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL,
                0o644,
            )?;
            file.write_all(bytes)?;
            file.sync_all()?;
            // linkat publishes complete bytes and refuses every existing destination.
            let code = unsafe {
                libc::linkat(
                    parent.as_raw_fd(),
                    temporary_c.as_ptr(),
                    parent.as_raw_fd(),
                    leaf_c.as_ptr(),
                    0,
                )
            };
            ensure!(
                code == 0,
                "destination appeared during initialization: {path}: {}",
                std::io::Error::last_os_error()
            );
            parent.sync_all()?;
            Ok(())
        })();
        let removed = unsafe { libc::unlinkat(parent.as_raw_fd(), temporary_c.as_ptr(), 0) };
        result?;
        ensure!(
            removed == 0,
            "temporary link cleanup failed for {path}: {}",
            std::io::Error::last_os_error()
        );
        parent.sync_all()?;
        Ok(())
    }

    pub fn append(&self, path: &str, expected: &[u8], suffix: &[u8]) -> Result<()> {
        let (parent, leaf) = self
            .parent(path, false)?
            .context("destination parent disappeared")?;
        let mut file = open(
            &parent,
            &leaf,
            libc::O_RDWR | libc::O_APPEND | libc::O_NONBLOCK,
            0,
        )?;
        ensure!(
            unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0,
            "destination busy: {path}"
        );
        let metadata = file.metadata()?;
        ensure!(
            metadata.is_file() && metadata.nlink() == 1 && metadata.len() == expected.len() as u64,
            "destination changed during initialization: {path}"
        );
        let mut actual = Vec::new();
        file.rewind()?;
        (&mut file)
            .take(expected.len() as u64 + 1)
            .read_to_end(&mut actual)?;
        ensure!(
            actual == expected,
            "destination changed during initialization: {path}"
        );
        file.write_all(suffix)?;
        file.sync_all()?;
        parent.sync_all()?;
        Ok(())
    }
}
