mod cli;

mod types;
pub use types::*;

mod config;
mod efi;
mod util;

mod command;

use clap::Parser;

use crate::cli::{Cli, Commands};

fn main() -> miette::Result<()> {
    let args = Cli::parse();

    util::check_privileges()?;

    match args.command {
        Commands::Boot { target, no_reboot } => {
            command::boot::boot(args.config_path, target, no_reboot)
        }
        Commands::Scan => command::scan::scan(args.config_path),
        Commands::Alias { action } => match action {
            cli::AliasCommands::List => command::alias::list(args.config_path),
            cli::AliasCommands::Add { name, id } => {
                command::alias::add(args.config_path, &name, id)
            }
            cli::AliasCommands::Remove { name } => command::alias::remove(args.config_path, &name),
            cli::AliasCommands::Clear { yes } => command::alias::clear(args.config_path, yes),
        },
    }
}
