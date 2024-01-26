use crate::config::MentalConfig;
use clap::Parser;
use std::path::Path;
use mapping::Mapping;

mod cli;
mod components;
mod config;
mod util;
mod mapping;

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
            let mappings: Vec<Mapping> = mapping::create_mapping(target_path, &mental_config);
            mental_config.mappings = mappings;
            println!("{:?}", mental_config);
            mental_config
                .dump(&config_file.to_path_buf())
                .expect("Error writing config")
        }
        Some(cli::Commands::Apply { target }) => {
            let target_paths = match target {
                None => mental_config.list_mapping_targets(),
                Some(target_folder) => vec![target_folder.to_owned()],
            };

            mental_config
                .write_components_to_folder(target_paths)
                .expect("Error")
        }
        None => {}
    }
}
