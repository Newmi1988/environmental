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

fn create_mapping(path: &Path, mental_config: &MentalConfig) -> Vec<Mapping> {
    let selected_folders = match cli::folder_multiselect(path) {
        Ok(selection) => selection,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Selected folders {:?}", selected_folders);

    let mut mappings: Vec<Mapping> = Vec::new();
    for f in selected_folders {
        let message = format!(
            "Select components that shoudl be included in forlder '{}'. Components: ",
            f
        );
        let selected_components = match cli::module_multiselect(mental_config, &message) {
            Ok(selection) => selection,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        let mut pathbuff = PathBuf::new();
        pathbuff.push(f);
        mappings.push(Mapping {
            path: pathbuff,
            components: selected_components.clone(),
        });
    }
    mappings
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
            config_path
        }
    };

    // load the config
    let mut mental_config = match config::MentalConfig::from_file(&config_file) {
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
            println!("Existing components:");
            for c in components {
                println!("  {}", &c)
            }
        }
        Some(cli::Commands::Map { target }) => {
            let target_path = match target {
                None => match config_file.parent() {
                    Some(parent) => parent,
                    None => panic!("Could not resolve folder"),
                },
                Some(target_folder) => target_folder,
            };
            let mappings: Vec<Mapping> = create_mapping(target_path, &mental_config);
            // for m in &mappings {
            //      let config_env: Vec<String> =
            //          mental_config.to_env(m.components);
            //      println!("Resulting env variables for folder: {:?}",&m.path);
            //      for env_entry in config_env {
            //          println!("{}", env_entry);
            //      }
            // }
            mental_config.mappings = mappings;
            println!("{:?}", mental_config);
            mental_config
                .dump(&config_file.to_path_buf())
                .expect("Error writing config")
        }
        None => {}
        _ => {}
    }
}
