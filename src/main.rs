use crate::{config::MentalConfig, mapping::MentalMapping};
use clap::Parser;
use mapping::FileIO;
use std::path::{Path, PathBuf};

mod cli;
mod components;
mod config;
mod mapping;
mod util;

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
    let mental_config = match config::MentalConfig::from_file(&config_file) {
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
        Some(cli::Commands::Component { component }) => match component {
            cli::Component::List {} => {
                let components = mental_config.list_components();
                println!("Existing components:");
                for c in components {
                    println!("  {}", &c)
                }
            }
            cli::Component::Create {
                name,
                prefix,
                keys,
                values,
            } => {
                println!("Creating component");

                if keys.len() == 0 && values.len() == 0 {
                    panic!("No keys and values given. Aborting creation of component")
                }

                if keys.len() != values.len() {
                    panic!("Number of keys and values is not equal.")
                }
                let mut key_values: Vec<(String, String)> = Vec::new();
                for (key, value) in keys.iter().zip(values.iter()) {
                    key_values.push((key.to_owned(), value.to_owned()))
                }

                match prefix {
                    Some(prefix) => {
                        match mental_config.create_component_with_prefix(
                            name.to_owned(),
                            prefix.to_owned(),
                            key_values,
                        ) {
                            Ok(config) => {
                                println!("Created component with prefix");
                                config
                                    .dump(&config_file.to_path_buf())
                                    .expect("Error writing config");
                            }
                            Err(err) => panic!("Error creating component {}", err),
                        }
                    }
                    None => match mental_config.create_component(name.to_owned(), key_values) {
                        Ok(config) => {
                            println!("Created component");
                            config
                                .dump(&config_file.to_path_buf())
                                .expect("Error writing config");
                        }
                        Err(err) => panic!("Error creating component {}", err),
                    },
                };
            }
        },
        Some(cli::Commands::Map { target }) => {
            let target_path = match target {
                None => match config_file.parent() {
                    Some(parent) => parent,
                    None => panic!("Could not resolve folder"),
                },
                Some(target_folder) => target_folder,
            };
            let mappings: MentalMapping =
                MentalMapping::new(target_path, mental_config.list_components());

            let mapping_name: String =
                match inquire::Text::new("Plase select a name for the mapping").prompt() {
                    Ok(name) => format!("./{}.map", name),
                    Err(_) => panic!("Not a valid name"),
                };

            let mut mapping_file = PathBuf::new();
            mapping_file.push(mapping_name);
            mappings.dump(&mapping_file).expect("Error writing config")
        }
        Some(cli::Commands::Apply { mapping, target }) => {
            let loaded_mapping = match MentalMapping::from_file(&mapping.as_path()) {
                Ok(m) => m,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };
            let target_paths = match target {
                None => loaded_mapping.list_targets(),
                Some(target_folder) => vec![target_folder.to_owned()],
            };

            loaded_mapping
                .apply(&mental_config, target_paths)
                .expect("Error")
        }
        None => {}
    }
}
