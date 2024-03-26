use crate::util;
use clap::{Parser, Subcommand};
use inquire::error::InquireResult;
use inquire::MultiSelect;
use std::path::{Path, PathBuf};

fn format_multiline_list(options: Vec<String>, message: &str) -> InquireResult<Vec<String>> {
    MultiSelect::new(message, options).prompt()
}

pub(crate) fn folder_multiselect(folder_path: &Path) -> InquireResult<Vec<String>> {
    let options = util::list_folders(folder_path);
    let folders_as_string: Vec<String> = options
        .into_iter()
        .filter_map(|f| f.into_os_string().into_string().ok())
        .collect();
    format_multiline_list(folders_as_string, "Select target folders:")
}

pub(crate) fn module_multiselect(
    components: Vec<String>,
    message: &str,
) -> InquireResult<Vec<String>> {
    format_multiline_list(components, message)
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub(crate) struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub(crate) config: Option<PathBuf>,

    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Sync to config to files
    Apply {
        mapping: PathBuf,
        target: Option<PathBuf>,
    },
    /// Map components to targets
    Map { target: Option<PathBuf> },
    /// Dump Schema
    Schema {
        /// Sets a custom config file
        #[arg(short, long, value_name = "PATH")]
        target: Option<PathBuf>,
    },
    /// List components
    Component {
        #[command(subcommand)]
        component: Component,
    },
}

#[derive(Subcommand, Debug)]
pub(crate) enum Component {
    List {},
    Create {
        #[arg(short, long, value_name = "name")]
        name: String,

        #[arg(short, long, value_name = "prefix")]
        prefix: Option<String>,

        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        keys: Vec<String>,

        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        values: Vec<String>,
    },
}
