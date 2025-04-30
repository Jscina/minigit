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
        Some(Command::Add { paths }) => {
            if let Err(e) = minigit::commands::add::run(paths) {
                eprintln!("Error adding file: {}", e);
            }
        }
        Some(Command::Commit { message }) => {
            if let Err(e) = minigit::commands::commit::run(message) {
                eprintln!("Error committing changes: {}", e);
            }
        }
        Some(Command::Log) => {
            if let Err(e) = minigit::commands::log::run() {
                eprintln!("Error displaying log: {}", e);
            }
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}
