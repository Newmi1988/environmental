//! Structs and implementation to handle components

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Struct holding the key and values
///
/// * `name`: name of the value
/// * `value`: value
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct KeyValue {
    name: String,
    value: String,
}

impl KeyValue {
    /// Format the key value to .env format
    fn to_env(&self) -> String {
        format!(r#"{0}="{1}""#, self.name, self.value)
    }
}

/// Component struct
///
/// * `name`: name of the component
/// * `prefix`: optional prefix put in front of the variable
/// * `values`: values under the component
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Component {
    pub(crate) name: String,
    prefix: Option<String>,
    values: Vec<KeyValue>,
}

impl Component {
    /// Format the component into the .env format
    pub(crate) fn to_env(&self) -> Vec<String> {
        let prefix_upper = match &self.prefix {
            Some(x) => format!("{}_", x.to_uppercase()),
            None => "".to_string(),
        };
        let mut formatted_values: Vec<String> = Vec::new();
        let component_comment = format!("# component {}", &self.name);
        formatted_values.push(component_comment);
        for value in &self.values {
            let value_with_prefix = format!(r#"{}{}"#, prefix_upper, value.to_env());
            formatted_values.push(value_with_prefix);
        }
        formatted_values
    }

    /// Create a component
    ///
    /// * `name`: name of the component
    /// * `prefix`: prefix of the component
    /// * `values`: collection of values
    pub(crate) fn new(
        name: String,
        prefix: Option<String>,
        values: Vec<(String, String)>,
    ) -> Component {
        let mut given_key_values: Vec<KeyValue> = Vec::new();
        for (key, value) in values {
            given_key_values.push(KeyValue { name: key, value })
        }
        Component {
            name,
            prefix,
            values: given_key_values,
        }
    }
}
