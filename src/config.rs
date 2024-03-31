use crate::components::Component;
use crate::mapping::FileIO;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::error::Error;
use std::fmt;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Config Struct
///
/// * `components`: collection of components
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MentalConfig {
    components: Vec<Component>,
}

impl FileIO for MentalConfig {}

/// Custom error used with config
#[derive(Debug, Clone)]
struct ConfigError(String);

impl fmt::Display for ConfigError {
    /// Format the custom error
    ///
    /// * `f`: formatter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid config option: {}", self.0)
    }
}

impl Error for ConfigError {}

impl MentalConfig {
    /// Serialize data into .env format
    ///
    /// * `component_keys`: slice of component keys
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

    /// Dump a json schema for validation
    ///
    /// * `target`: filepath to save the schema into
    pub(crate) fn create_schema(target: &PathBuf) -> std::io::Result<()> {
        let schema = schema_for!(MentalConfig);
        let formatted_schema = serde_json::to_string(&schema).expect("Error creating schema");
        let mut file = File::create(target)?;
        file.write_all(formatted_schema.as_bytes())?;
        Ok(())
    }

    /// List names of all defined components
    pub(crate) fn list_components(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for c in &self.components {
            res.push(c.name.clone())
        }
        res
    }

    /// Load config from file
    ///
    /// * `config_file`: path to load the config from
    pub fn from_file(config_file: &&Path) -> Result<MentalConfig, Box<dyn Error>> {
        let config_input = read_to_string(config_file)?;
        let config: MentalConfig = from_str(&config_input)?;
        Ok(config)
    }

    /// Check if a component with a given name exists
    ///
    /// * `name`: name to search for
    pub fn name_exists(&self, name: &String) -> bool {
        for comp in &self.components {
            if name == &comp.name {
                return true;
            }
        }
        false
    }

    /// Create a new component
    ///
    /// * `name`: name of the component
    /// * `values`: values of the component
    pub fn create_component(
        mut self,
        name: String,
        values: Vec<(String, String)>,
    ) -> Result<Self, Box<dyn Error>> {
        if self.name_exists(&name) {
            Err(Box::new(ConfigError("Name already exists".into())))
        } else {
            self.components.push(Component::new(name, None, values));
            Ok(self)
        }
    }

    /// Create a new component
    ///
    /// * `name`: name of the component
    /// * `prefix`: prefix to prefix the variables with
    /// * `values`: values of the component
    pub fn create_component_with_prefix(
        mut self,
        name: String,
        prefix: String,
        values: Vec<(String, String)>,
    ) -> Result<Self, Box<dyn Error>> {
        if self.name_exists(&name) {
            Err(Box::new(ConfigError("Name already exists".into())))
        } else {
            self.components
                .push(Component::new(name, Some(prefix), values));
            Ok(self)
        }
    }
}
