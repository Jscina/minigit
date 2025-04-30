use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Init,
    Add {
        #[arg(short, long, value_name = "FILE_OR_DIR")]
        path: String,
    },
    Commit {
        #[arg(short, long)]
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
