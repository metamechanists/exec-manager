use clap::{Parser, Subcommand};
use commands::{deploy, integrity, link, list, unlink, update, verify};
use metadata::MetaData;
use plugin_data::PluginData;

mod commands;
mod messages;
mod metadata;
mod plugin_data;

#[derive(Clone, Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    /// Update a specific plugin. 'all', 'paper', and 'velocity' are valid.
    Update { plugin: String },
    /// Deploy a specific plugin from the dev server. 'all' is valid.
    Deploy { plugin: String },
    /// List servers a plugin is linked to
    List { plugin: String },
    /// Link a plugin to a server. 'paper' and 'velocity' are valid.
    Link { plugin: String, server: String },
    /// Unlink a plugin from a server. 'paper' and 'velocity' are valid.
    Unlink { plugin: String, server: String },
    /// Check that the executables directory contains all required executables (including paper and velocity)
    Verify,
    /// Check that the symlinks in all servers are valid
    Integrity,
}

fn main() {
    let cli = Cli::parse();
    let plugin_data = PluginData::load();
    let metadata = MetaData::load();
    match cli.command {
        Commands::Update { plugin } => update(&plugin_data, &metadata, plugin),
        Commands::Deploy { plugin } => deploy(&plugin_data, &metadata, plugin),
        Commands::List { plugin } => list(&metadata, plugin),
        Commands::Link { plugin, server } => link(&metadata, &plugin_data, plugin, server),
        Commands::Unlink { plugin, server } => unlink(&metadata, plugin, server),
        Commands::Verify => verify(&metadata, &plugin_data),
        Commands::Integrity => integrity(&metadata),
    }
}
