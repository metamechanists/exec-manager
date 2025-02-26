use std::fs;

use crate::{messages, metadata::MetaData};

fn check_link(path: String, server: String) {
    if fs::symlink_metadata(path).is_ok() {
        println!("{}", messages::linked_list_item(server));
    }
}

pub fn list_plugin(metadata: &MetaData, plugin: String) {
    for server in metadata.get_servers() {
        let path = metadata.get_server_directory(&server) + "/plugins/" + plugin.as_str() + ".jar";
        check_link(path, server);
    }
}

pub fn list_paper(metadata: &MetaData) {
    for server in metadata.get_servers() {
        let path = metadata.get_server_directory(&server) + "/paper.jar";
        check_link(path, server);
    }
}

pub fn list_velocity(metadata: &MetaData) {
    for server in metadata.get_servers() {
        let path = metadata.get_server_directory(&server) + "/velocity.jar";
        check_link(path, server);
    }
}
