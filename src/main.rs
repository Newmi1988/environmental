use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod components;
mod config;

#[derive(Debug, Serialize, Deserialize)]
struct Mapping {
    path: PathBuf,
    components: Vec<String>,
}

fn main() {
    let config = config::load_config();
    println!("{:?}", &config);

    let config_env: Vec<String> = config.to_env(vec!["postgres".to_string(), "test".to_string()]);
    println!("{:?}", &config_env)
}
