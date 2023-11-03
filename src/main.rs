mod commands;
mod spotify;
mod util;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    #[arg(default_value = "~/.config/spotify-cli/")]
    config: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Playlist {
        #[command(subcommand)]
        playlist_command: PlaylistCommands,
    },
    User,
    Auth,
    Debug {
        #[command(subcommand)]
        debug_command: DebugSub,
    },
}

#[derive(Subcommand)]
enum PlaylistCommands {
    #[command(about = "Copy a playlist's contents into another")]
    Copy {
        #[arg(value_name = "Playlist 1")]
        a: String,

        #[arg(value_name = "Preposition")]
        /// Direction to copy
        ///
        /// Copying A into B modifies B, using contents of A
        ///
        /// Copying A from B modifies A, using contents of B
        prep: commands::playlist::copy::CopyPreposition,

        #[arg(value_name = "Playlist 2")]
        b: String,
    },
    Sort {
        #[arg(value_name = "Playlist")]
        playlist: String,

        #[arg(value_name = "Sorting method")]
        sort_method: Vec<commands::playlist::sort::SortMethod>,
    },
}

#[derive(Subcommand)]
enum DebugSub {
    /// Dumps the contents of a playlist to stdout
    DumpPlaylist {
        #[arg(value_name = "Playlist")]
        playlist: String,
    },
    /// Empties the contents of a playlist
    ClearPlaylist {
        #[arg(value_name = "Playlist")]
        playlist: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Playlist { playlist_command } => match playlist_command {
            PlaylistCommands::Copy { a, prep, b } => {
                commands::playlist::copy::run(cli.config, a, prep, b);
            }
            PlaylistCommands::Sort {
                playlist,
                sort_method,
            } => {
                commands::playlist::sort::run(cli.config, playlist, sort_method);
            }
        },
        Commands::User => {
            unimplemented!()
        }
        Commands::Auth => {
            commands::auth::run(cli.config);
        }
        Commands::Debug { debug_command } => {
            // Match the debug subcommands
            match debug_command {
                DebugSub::DumpPlaylist { playlist } => {
                    commands::debug::dump_playlist(cli.config, playlist);
                }
                DebugSub::ClearPlaylist { playlist } => {
                    commands::debug::clear_playlist(cli.config, playlist);
                }
            }
        }
    }
}
