use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct KeyValue {
    name: String,
    value: String,
}

impl KeyValue {
    fn to_env(&self) -> String {
        format!(r#"{0}="{1}""#, self.name, self.value)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Component {
    pub(crate) name: String,
    prefix: Option<String>,
    values: Vec<KeyValue>,
}

impl Component {
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
