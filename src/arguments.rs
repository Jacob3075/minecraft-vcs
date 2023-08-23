use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// initialize folder structure on remote
    INIT,
    /// get latest version of world from remote
    PULL,
    /// push latest version of world to remote
    PUSH,
    /// get details for saved worlds on remote
    LOGS,
}
