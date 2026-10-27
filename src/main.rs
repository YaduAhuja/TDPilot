mod cli;
mod model;
mod utils;
mod store;
mod error;

use clap::Parser;
use cli::{Cli, Commands, profile::ProfileCommand};
use store::Store;
use error::TDAppError;

use model::Profile;

fn main() {
    let store = Store {
        path : String::from("./profiles")
    };

    if let Some(err) = handle_cli(&store).err() {
        println!("Unable to complete command: {}", err);
    };
}

fn handle_cli(store: &Store) -> Result<(), TDAppError> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Profile(args) => match args.command {
            ProfileCommand::Save(args) => {
                println!("Save profile: {}", args.name);
                println!("JSON: {}", args.json);
                let obj = serde_json::from_str::<model::RyzenAdjConfig>(&args.json)?;
                println!("Json Parsed : {:?}", obj);
                let string = serde_json::to_string(&obj)?;
                println!("Json String : {:?}", string);
                let profile = model::Profile {name: args.name, config: Some(obj.clone())};
                store.save_profile(profile)?;
                println!("Config : {:?}", obj.to_args().unwrap());
                println!("Force overwrite: {}", args.force);
            }
            ProfileCommand::Apply(args) => {
                println!("Apply profile: {}", args.name);
            }
            ProfileCommand::Delete(args) => {
                println!("Delete profile: {}", args.name);
                // TODO: Add the confirmation logic
                // println!("Skip confirmation: {}", args.yes);
                store.delete_profile(Profile{name : args.name, config: None})?;
            }
            ProfileCommand::List => {
                let profiles = store.list_profiles()?;
                println!("Available Profiles: {:?}", profiles);
            }
            ProfileCommand::Show(args) => {
                println!("Show profile: {}", args.name);
                let profile = store.parse_profile(Profile { name: args.name, config: None })?;
                println!("Profile: {:?}", profile);
            }
        },
        Commands::Status => {

        }
    }
    Ok(())
}