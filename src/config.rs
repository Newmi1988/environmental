use crate::components::Component;
use crate::Mapping;
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::fs::read_to_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct MentalConfig {
    components: Vec<Component>,
    mappings: Vec<Mapping>,
}

impl MentalConfig {
    pub(crate) fn to_env(&self, component_keys: Vec<String>) -> Vec<String> {
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
}

use std::error::Error;

pub fn load_config(config_file: &str) -> Result<MentalConfig, Box<dyn Error>> {
    let config_input = read_to_string(config_file)?;
    let config: MentalConfig = from_str(&config_input)?;
    Ok(config)
}
