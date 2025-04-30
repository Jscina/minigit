extern crate minigit;
use clap::Parser;
use minigit::{commands::init, Cli, Command};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Init) => {
            if let Err(e) = init::run() {
                eprintln!("Error initializing repository: {}", e);
            }
        }
        Some(Command::Add { path }) => {
            println!("Adding paths: {}", path);
        }
        Some(Command::Commit { message }) => {
            println!("Committing with message: {}", message);
        }
        Some(Command::Log) => {
            println!("Showing commit log...");
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}
