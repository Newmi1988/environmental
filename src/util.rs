use crate::util;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub(crate) fn folders(path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| Some(entry.ok()?.path().to_path_buf()))
        .filter(|path| path.is_dir())
        .collect())
}

pub(crate) fn list_folders(folder: &Path) -> Vec<PathBuf> {
    match folders(folder) {
        Ok(mut folder_paths) => {
            folder_paths.push(Path::new(folder)
                .to_path_buf());
            folder_paths
        },
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}
