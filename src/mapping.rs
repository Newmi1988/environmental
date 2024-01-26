use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::cli;
use crate::config::MentalConfig;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Mapping {
    pub(crate) path: PathBuf,
    pub(crate) components: Vec<String>,
}

pub fn create_mapping(path: &Path, mental_config: &MentalConfig) -> Vec<Mapping> {
    let selected_folders = match cli::folder_multiselect(path) {
        Ok(selection) => selection,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Selected folders {:?}", selected_folders);

    let mut mappings: Vec<Mapping> = Vec::new();
    for f in selected_folders {
        let message = format!(
            "Select components that shoudl be included in forlder '{}'. Components: ",
            f
        );
        let selected_components = match cli::module_multiselect(mental_config, &message) {
            Ok(selection) => selection,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        let mut pathbuff = PathBuf::new();
        pathbuff.push(f);
        mappings.push(Mapping {
            path: pathbuff,
            components: selected_components.clone(),
        });
    }
    mappings
}
