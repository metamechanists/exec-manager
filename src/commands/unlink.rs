use std::process::Command;

use crate::{messages, metadata::MetaData};

fn delete_symlink(to: String) {
    let output = Command::new("rm")
        .arg(to)
        .output();
    match output {
        Err(error) => println!("{}", messages::unlink_failed(error)),
        Ok(_) => (),
    }
}

pub fn unlink_plugin(metadata: &MetaData, plugin: String, server: String) {
    let to = metadata.get_server_directory(&server) + "/plugins/" + plugin.as_str() + ".jar";
    delete_symlink(to);
}

pub fn unlink_paper(metadata: &MetaData, server: String) {
    let to = metadata.get_server_directory(&server) + "/paper.jar";
    delete_symlink(to);
}

pub fn unlink_velocity(metadata: &MetaData, server: String) {
    let to = metadata.get_server_directory(&server) + "/velocity.jar";
    delete_symlink(to);
}
