use std::path::{Path, PathBuf};
use std::{fs, io};
use crate::util;

pub(crate) fn folders(path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| Some(entry.ok()?.path().strip_prefix(path).ok()?.to_path_buf()))
        .filter(|path| path.is_dir())
        .collect())
}

pub(crate) fn list_folders(folder: &str) -> Vec<PathBuf> {
    let parent_folder = match Path::new(folder).parent() {
        Some(path) => path,
        None => panic!("Problem getting parent directory"),
    };

    let folders = match util::folders(parent_folder) {
        Ok(folder_paths) => folder_paths,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    folders
}
