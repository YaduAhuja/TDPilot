mod cli;
mod config;
mod error;
mod model;
mod store;
mod utils;

use clap::Parser;
use cli::{Cli, Commands, profile::ProfileCommand};
use config::AppConfig;
use error::TDAppError;
use store::Store;

use model::Profile;
use std::{
    path::Path,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Unable to complete command: {}", err);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), TDAppError> {
    let app_config = AppConfig::load_or_create(Path::new("config.json"))?;
    let store = Store {
        path: app_config.profile_directory.clone(),
    };
    handle_cli(&store, &app_config)
}

fn handle_cli(store: &Store, app_config: &AppConfig) -> Result<(), TDAppError> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Profile(args) => match args.command {
            ProfileCommand::Save(args) => {
                let obj = serde_json::from_str::<model::RyzenAdjConfig>(&args.json)?;
                let profile = model::Profile {
                    name: args.name.clone(),
                    config: Some(obj),
                };
                store.save_profile(profile, args.force)?;
                println!("Saved profile '{}'.", args.name);
            }
            ProfileCommand::Apply(args) => {
                let profile = store.parse_profile(Profile {
                    name: args.name.clone(),
                    config: None,
                })?;
                let config = profile.config.ok_or_else(|| crate::error::TDCustomError {
                    message: format!("Profile '{}' has no settings.", args.name),
                })?;
                run_ryzenadj(&app_config.ryzen_adj_exec_path, &config.to_args()?)?;
            }
            ProfileCommand::Delete(args) => {
                if !args.yes {
                    eprintln!("Use --yes to confirm deletion of profile '{}'.", args.name);
                    return Ok(());
                }
                store.delete_profile(Profile {
                    name: args.name,
                    config: None,
                })?;
                println!("Deleted profile.");
            }
            ProfileCommand::List => {
                let profiles = store.list_profiles()?;
                for profile in profiles {
                    println!("{profile}");
                }
            }
            ProfileCommand::Show(args) => {
                let profile = store.parse_profile(Profile {
                    name: args.name,
                    config: None,
                })?;
                println!("{}", serde_json::to_string_pretty(&profile.config)?);
            }
        },
        Commands::Status => {
            run_ryzenadj(&app_config.ryzen_adj_exec_path, &["--info".to_owned()])?;
        }
    }
    Ok(())
}

fn run_ryzenadj(executable: &Path, args: &[String]) -> Result<(), TDAppError> {
    let output = Command::new(executable)
        .args(args)
        .output()
        .map_err(|error| TDAppError::CommandFailed {
            program: executable.display().to_string(),
            message: error.to_string(),
        })?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(TDAppError::CommandFailed {
            program: executable.display().to_string(),
            message: format!("process exited with status {}", output.status),
        });
    }
    Ok(())
}
