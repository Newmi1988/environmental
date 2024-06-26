use crate::util;
use clap::{Parser, Subcommand};
use inquire::error::InquireResult;
use inquire::MultiSelect;
use std::path::{Path, PathBuf};

pub(crate) fn format_multiline_list(
    options: Vec<String>,
    message: &str,
) -> InquireResult<Vec<String>> {
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

        /// only print to stdout
        #[arg(short, long, action)]
        stdout: bool,
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
    /// List existing components
    List {},

    /// Print components
    Show {
        /// Components to show (whitespace sperated)
        #[clap(value_parser, num_args =1.., value_delimiter = ' ', required=true)]
        names: Vec<String>,
    },

    /// Create a new component
    Create {
        /// Name of the component
        #[arg(value_name = "name")]
        name: String,

        /// optional prefix
        #[arg(value_name = "prefix")]
        prefix: Option<String>,

        /// keys, seperated by whitespace
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ', required=true)]
        keys: Vec<String>,

        /// values, seperated by whitespace
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ', required=true)]
        values: Vec<String>,
    },

    /// Create a component from current environment
    FromEnv {},
}
