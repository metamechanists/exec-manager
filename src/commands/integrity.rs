use std::fs;

use crate::metadata::MetaData;

pub fn integrity(metadata: &MetaData) {
    for server in metadata.get_servers() {
        let path = metadata.get_server_directory(&server) + "/plugins/";
        for plugin in fs::read_dir(path).expect("Failed to read path") {
            let plugin = plugin.expect("Failed to get direntry");
            if plugin.metadata().expect("Failed to get file metadata").is_symlink() && !plugin.path().exists() {
                println!("Found broken symlink {:?}", plugin.path());
            }
        }
    }
}
