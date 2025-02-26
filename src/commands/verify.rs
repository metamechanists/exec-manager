use std::{collections::HashMap, fs};

use crate::{messages, metadata::MetaData, plugin_data::PluginData};

fn check_jar_exists_and_not_empty(metadata: &MetaData, plugin: &String) {
    let url = metadata.get_executables_directory() + "/" + plugin + ".jar";
    if let Ok(file_metadata) = fs::metadata(&url) {
        if !file_metadata.is_file() {
            println!("{}", messages::jar_not_found(plugin));
            return;
        }
        if file_metadata.len() == 0 {
            println!("{}", messages::jar_empty(plugin))
        }
    }
}

pub fn verify(metadata: &MetaData, plugin_data: &HashMap<String, PluginData>) {
    for plugin in plugin_data.keys() {
        check_jar_exists_and_not_empty(metadata, plugin);
    }
    check_jar_exists_and_not_empty(metadata, &"paper.jar".to_string());
    check_jar_exists_and_not_empty(metadata, &"velocity.jar".to_string());
}
