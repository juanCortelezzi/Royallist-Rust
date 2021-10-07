use std::path::{Path, PathBuf};
use std::{fs, io::Result};

pub fn get_filename(path: &Path) -> String {
    match path.file_name() {
        Some(name) => name.to_os_string().into_string().unwrap(),
        None => panic!("couldnt get filename"),
    }
}

pub fn get_extension(path: &Path) -> String {
    match path.extension() {
        Some(name) => name.to_os_string().into_string().unwrap(),
        None => String::new(),
    }
}

/// read_dir reads the passed directory and returns a vec with its contents
pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;

    Ok(entries)
}
