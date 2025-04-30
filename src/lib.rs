use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Init,
    Add {
        #[arg(
            required = true,
            value_name = "FILE_OR_DIR",
            help = "Path to the file or directory to add"
        )]
        paths: Vec<String>,
    },
    Commit {
        #[arg(
            required = true,
            short,
            long,
            help = "Commit message describing the changes"
        )]
        message: String,
    },
    Log,
}

pub mod commands {
    pub mod add;
    pub mod commit;
    pub mod init;
    pub mod log;
}

pub mod core {
    pub mod object;
    pub mod repo;
    pub mod utils;
}
