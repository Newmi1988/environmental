use crate::components::Component;
use crate::Mapping;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::error::Error;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MentalConfig {
    components: Vec<Component>,
    pub mappings: Vec<Mapping>,
}

impl MentalConfig {
    pub(crate) fn to_env(&self, component_keys: &[String]) -> Vec<String> {
        let filtered_components: Vec<&Component> = self
            .components
            .iter()
            .filter(|c| component_keys.contains(&c.name))
            .collect();

        let mut combined_values: Vec<String> = Vec::new();
        for c in filtered_components {
            let component_output = c.to_env();
            for x in component_output {
                combined_values.push(x);
            }
        }
        combined_values
    }

    pub(crate) fn create_schema(target: &PathBuf) -> std::io::Result<()> {
        let schema = schema_for!(MentalConfig);
        let formatted_schema = serde_json::to_string(&schema).expect("Error creating schema");
        let mut file = File::create(target)?;
        file.write_all(formatted_schema.as_bytes())?;
        Ok(())
    }

    pub(crate) fn list_components(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for c in &self.components {
            res.push(c.name.clone())
        }
        res
    }

    pub(crate) fn list_mapping_targets(&self) -> Vec<PathBuf> {
        let mut res: Vec<PathBuf> = Vec::new();
        for m in &self.mappings {
            res.push(m.path.clone())
        }
        res
    }

    pub(crate) fn write_components_to_folder(&self, targets: Vec<PathBuf>) -> std::io::Result<()> {
        for m in &self.mappings {
            if targets.contains(&m.path) {
                let target_config_env = self.to_env(&m.components);
                println!("Resulting env variables for folder: {:?}", &m.path);
                for env_entry in &target_config_env {
                    println!("  {}", env_entry);
                }
                let formated_path = format!("{}{}", &m.path.display(), "/.env");
                let target_path = PathBuf::from(formated_path);
                fs::write(target_path, target_config_env.join("\n"))?;
            }
        }
        Ok(())
    }

    pub(crate) fn dump(&self, target: &PathBuf) -> std::io::Result<()> {
        let mut file = File::create(target)?;
        let config_as_string = serde_yaml::to_string(self).expect("Error writing config");
        file.write_all(config_as_string.as_bytes())?;
        Ok(())
    }

    pub fn from_file(config_file: &&Path) -> Result<MentalConfig, Box<dyn Error>> {
        let config_input = read_to_string(config_file)?;
        let config: MentalConfig = from_str(&config_input)?;
        Ok(config)
    }
}
