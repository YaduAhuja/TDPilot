pub mod profile;
pub mod status;

use clap::{Parser, Subcommand};

/// TDPilot — Linux Ryzen CPU tuning utility
#[derive(Parser, Debug)]
#[command(
    name = "tdpilot",
    version,
    about = "Manage AMD Ryzen CPU power and performance profiles",
    disable_help_subcommand = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage CPU tuning profiles
    Profile(profile::ProfileArgs),
    Status,
}