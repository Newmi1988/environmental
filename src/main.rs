use serde::{Serialize, Deserialize};
use serde_yaml;
use std::include_str;
use serde_yaml::from_str;

#[derive(Debug, Serialize, Deserialize)]
struct KeyValue {
    name: String,
    value: String
}

impl KeyValue {
    fn to_env(&self) -> String {
       return format!("{0}={1}",self.name,self.value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Component {
    name: String,
    prefix: Option<String>,
    values: Vec<KeyValue>
}

impl Component {
    fn to_env(&self) -> Vec<String> {
        let prefix_upper = match &self.prefix {
            Some(x) => x.to_uppercase(),
            None => "".to_string()
        };
        let mut formated_values: Vec<String> = Vec::new();
        for value in &self.values {
           formated_values.push(value.to_env())
        }
        formated_values
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MentalConfig {
    components: Vec<Component>,
    mapping: Vec<KeyValue>
}

fn load_config() -> MentalConfig {
    let config_input = include_str!("../mental.yaml");
    let config : MentalConfig = from_str(config_input).unwrap();
    config
}


fn main() {
    let config = load_config();
    println!("{:?}",&config);
}
