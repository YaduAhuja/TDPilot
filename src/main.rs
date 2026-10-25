mod cli;

use clap::Parser;
use cli::{Cli, Commands, ProfileCommand};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Profile(args) => match args.command {
            ProfileCommand::Save(args) => {
                println!("Save profile: {}", args.name);
                println!("JSON: {}", args.json);
                println!("Force overwrite: {}", args.force);
            }
            ProfileCommand::Apply(args) => {
                println!("Apply profile: {}", args.name);
            }
            ProfileCommand::Delete(args) => {
                println!("Delete profile: {}", args.name);
                println!("Skip confirmation: {}", args.yes);
            }
            ProfileCommand::List => {
                println!("List profiles");
            }
            ProfileCommand::Show(args) => {
                println!("Show profile: {}", args.name);
            }
        },
    }
}