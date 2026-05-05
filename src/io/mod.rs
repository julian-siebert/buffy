use std::path::{Path, PathBuf};

pub use crate::io::error::Error;

mod error;

pub fn create_dir_all(path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();
    std::fs::create_dir_all(path).map_err(|e| error::from_io(path, e))
}

pub fn remove_dir_all(path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();
    std::fs::remove_dir_all(path).map_err(|e| error::from_io(path, e))
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let path = path.as_ref();
    std::fs::read(path).map_err(|e| error::from_io(path, e))
}

pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let path = path.as_ref();
    std::fs::read_to_string(path).map_err(|e| error::from_io(path, e))
}

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<(), Error> {
    let path = path.as_ref();

    std::fs::write(path, contents).map_err(|e| error::from_io(path, e))
}

pub fn exists<P: AsRef<Path>>(path: P) -> Result<bool, Error> {
    let path = path.as_ref();

    std::fs::exists(path).map_err(|e| error::from_io(path, e))
}

pub fn ensure_dir(path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();
    if path.is_dir() {
        return Ok(());
    }
    Err(Error::NotADirectory {
        path: path.to_path_buf(),
    })
}

pub fn read_dir(path: impl AsRef<Path>) -> Result<ReadDir, Error> {
    let path = path.as_ref();
    let inner = std::fs::read_dir(path).map_err(|e| error::from_io(path, e))?;
    Ok(ReadDir {
        inner,
        path: path.to_owned(),
    })
}

pub struct ReadDir {
    inner: std::fs::ReadDir,
    path: PathBuf,
}

impl Iterator for ReadDir {
    type Item = Result<PathBuf, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|res| {
            res.map(|entry| entry.path())
                .map_err(|e| error::from_io(&self.path, e))
        })
    }
}
