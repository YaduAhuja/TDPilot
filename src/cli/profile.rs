use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct ProfileArgs {
    #[command(subcommand)]
    pub command: ProfileCommand,
}

#[derive(Subcommand, Debug)]
pub enum ProfileCommand {
    /// Save a profile from JSON values
    Save(SaveProfileArgs),

    /// Apply a saved profile using RyzenAdj
    Apply(ProfileNameArgs),

    /// Delete a saved profile
    Delete(DeleteProfileArgs),

    /// List saved profiles
    List,

    /// Display a saved profile
    Show(ProfileNameArgs),
}

#[derive(Args, Debug)]
pub struct SaveProfileArgs {
    /// Profile name, e.g. balanced or performance
    pub name: String,

    /// RyzenAdj settings as a JSON object
    #[arg(long, value_name = "JSON")]
    pub json: String,

    /// Replace an existing profile
    #[arg(long, default_value_t = false)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct ProfileNameArgs {
    /// Name of the profile
    pub name: String,
}

#[derive(Args, Debug)]
pub struct DeleteProfileArgs {
    /// Name of the profile
    pub name: String,

    /// Delete without interactive confirmation
    #[arg(long, short)]
    pub yes: bool,
}