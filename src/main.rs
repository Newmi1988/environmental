use crate::config::MentalConfig;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

mod cli;
mod components;
mod config;
mod util;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct Mapping {
    path: PathBuf,
    components: Vec<String>,
}

fn main() {
    let config_file = "./mental.yaml";
    let config = match config::load_config(&config_file) {
        Ok(config) => config,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("{:?}", &config);

    let ans = match cli::folder_multiselect(config_file) {
        Ok(selection) => selection,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("{:?}", ans);

    let config_env: Vec<String> = config.to_env(vec!["postgres".to_string(), "test".to_string()]);
    println!("{:?}", &config_env);

    MentalConfig::create_schema("./.mental.schema.json").expect("Error writing file");
}
