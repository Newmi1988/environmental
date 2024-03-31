use crate::cli;
use crate::config::MentalConfig;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::error::Error;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
/// Mapping from components to path
///
/// * `path`: target path the variables are mapped into
/// * `components`: the components that should be mapped
pub struct Mapping {
    pub(crate) path: PathBuf,
    pub(crate) components: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
/// Collection of mappings
///
/// * `mappings`: collection of mappings
pub struct MentalMapping {
    pub mappings: Vec<Mapping>,
}

/// trait for handling that handles Serialization and Deserialiation of structs
pub trait FileIO: serde::Serialize {
    /// Dump the struct into a file
    ///
    /// * `target`: target file to write into
    fn dump(&self, target: &PathBuf) -> std::io::Result<()> {
        let mut file = File::create(target)?;
        let struct_as_string = serde_yaml::to_string(self).expect("Error writing config");
        file.write_all(struct_as_string.as_bytes())?;
        Ok(())
    }

    /// Load the given struct from a file
    ///
    /// * `struct_file`: file that contains the struct in a serialized format
    fn from_file(struct_file: &&Path) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
        for<'a> Self: Deserialize<'a>,
    {
        let struct_input = read_to_string(struct_file)?;
        let deserialized_struct: Self = from_str(&struct_input)?;
        Ok(deserialized_struct)
    }
}

impl FileIO for MentalMapping {}

impl MentalMapping {
    pub fn new(path: &Path, components: Vec<String>) -> MentalMapping {
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
            let selected_components = match cli::module_multiselect(components.clone(), &message) {
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
        MentalMapping { mappings }
    }

    pub(crate) fn apply(
        &self,
        config: &MentalConfig,
        targets: Vec<PathBuf>,
        to_stdout: &bool,
    ) -> std::io::Result<()> {
        for m in &self.mappings {
            if targets.contains(&m.path) {
                let target_config_env = config.to_env(&m.components);
                if *to_stdout {
                    for env_entry in &target_config_env {
                        println!("  {}", env_entry);
                    }
                } else {
                    let formatted_path = format!("{}{}", &m.path.display(), "/.env");
                    let target_path = PathBuf::from(formatted_path);
                    fs::write(target_path, target_config_env.join("\n"))?;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn list_targets(&self) -> Vec<PathBuf> {
        let mut res: Vec<PathBuf> = Vec::new();
        for m in &self.mappings {
            res.push(m.path.clone())
        }
        res
    }
}
