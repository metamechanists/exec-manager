use std::process::Command;

use crate::{messages, metadata::MetaData};

fn create_symlink(from: String, to: String) {
    let output = Command::new("ln")
        .arg("-s")
        .arg(from)
        .arg(to)
        .output();
    match output {
        Err(error) => println!("{}", messages::link_failed(error)),
        Ok(_) => (),
    }
}

pub fn link_plugin(metadata: &MetaData, plugin: String, server: String) {
    let from = metadata.get_executables_directory() + "/" + plugin.as_str() + ".jar";
    let to = metadata.get_server_directory(&server) + "/plugins";
    create_symlink(from, to);
}

pub fn link_paper(metadata: &MetaData, server: String) {
    let from = metadata.get_executables_directory() + "/paper.jar";
    let to = metadata.get_server_directory(&server);
    create_symlink(from, to);
}

pub fn link_velocity(metadata: &MetaData, server: String) {
    let from = metadata.get_executables_directory() + "/velocity.jar";
    let to = metadata.get_server_directory(&server);
    create_symlink(from, to);
}
