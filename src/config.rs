use crate::components::Component;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::error::Error;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MentalConfig {
    components: Vec<Component>,
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

    pub fn from_file(config_file: &&Path) -> Result<MentalConfig, Box<dyn Error>> {
        let config_input = read_to_string(config_file)?;
        let config: MentalConfig = from_str(&config_input)?;
        Ok(config)
    }

    pub fn create_component(
        mut self,
        name: String,
        values: Vec<(String, String)>,
    ) -> Result<(), Box<dyn Error>> {
        self.components.push(Component::new(name, None, values));
        Ok(())
    }

    pub fn create_component_with_prefix(
        mut self,
        name: String,
        prefix: String,
        values: Vec<(String, String)>,
    ) -> Result<(), Box<dyn Error>> {
        self.components
            .push(Component::new(name, Some(prefix), values));
        Ok(())
    }
}
