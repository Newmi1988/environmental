use crate::config::MentalConfig;
use clap::Parser;
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
    let cli = cli::Cli::parse();
    let config_file = match cli.config.as_deref() {
        None => {
            let config_file: &Path = Path::new("./mental.yaml");
            config_file
        }
        Some(config_path) => {
            println!("Setting config to :{}", config_path.display());
            let config_file = config_path;
            config_file
        }
    };

    // load the config
    let mental_config = match config::load_config(&config_file) {
        Ok(config) => config,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // match subcommands
    match &cli.command {
        Some(cli::Commands::Schema { target }) => match target {
            None => panic!("Target could not be resolved"),
            Some(target) => {
                MentalConfig::create_schema(target).expect("Error writing file");
            }
        },
        Some(cli::Commands::List {}) => {
            let components = mental_config.list_components();
            println!("Existing components: {:?}",&components)
        },
        None => {},
        _ => {}
    }


    // test implementations
    let ans = match cli::folder_multiselect(config_file) {
        Ok(selection) => selection,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("{:?}", ans);

    let config_env: Vec<String> = mental_config.to_env(vec!["postgres".to_string(), "test".to_string()]);
    println!("{:?}", &config_env);

}
