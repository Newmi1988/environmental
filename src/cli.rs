use inquire::error::InquireResult;
use inquire::{
    formatter::MultiOptionFormatter, list_option::ListOption, validator::Validation, InquireError,
    MultiSelect,
};
use std::path::PathBuf;
use crate::util;

fn format_multiline_list(options: Vec<String>, message : String) -> InquireResult<Vec<String>> {
    let ans = MultiSelect::new(&*message, options)
        .prompt();
    ans
}

pub(crate) fn folder_multiselect(config_file_path : &str) -> Result<InquireResult<Vec<String>>, InquireError> {
    let options = util::list_folders(config_file_path);
    let folders_as_string : Vec<String> = options
        .into_iter()
        .filter_map(|f| {
           f.into_os_string().into_string().ok()
        })
        .collect();
    Ok(format_multiline_list(folders_as_string,"Select foldres:".to_string()))
}
