use crate::{util, config::MentalConfig};
use inquire::error::InquireResult;
use inquire::MultiSelect;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};

fn format_multiline_list(options: Vec<String>, message: &str) -> InquireResult<Vec<String>> {
    let ans = MultiSelect::new(message, options).prompt();
    ans
}

pub(crate) fn folder_multiselect(folder_path: &Path) -> InquireResult<Vec<String>> {
    let options = util::list_folders(folder_path);
    let folders_as_string: Vec<String> = options
        .into_iter()
        .filter_map(|f| f.into_os_string().into_string().ok())
        .collect();
    format_multiline_list(
        folders_as_string,
        "Select folders:",
    )
}

pub(crate) fn module_multiselect(mental_config : &MentalConfig) -> InquireResult<Vec<String>> {
    let components = mental_config.list_components();
    format_multiline_list(
        components,
        "Select components that shoudl be included in folder",
    )
}


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub(crate) config: Option<PathBuf>,

    /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,

    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Sync to config to files
    Sync {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// Map components to targets
    Map {
        target: Option<PathBuf>
    },
    /// Dump Schema
    Schema {
        /// Sets a custom config file
        #[arg(short, long, value_name = "PATH")]
        target: Option<PathBuf>,
    },
    /// List components
    List {},
}
