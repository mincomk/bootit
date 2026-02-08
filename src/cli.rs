use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bootit")]
#[command(version)]
#[command(about = "Simple in-system boot selection utility", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, help = "Path to configuration file")]
    pub config_path: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Boot OS")]
    Boot {
        #[arg(help = "ID or alias for the next boot target")]
        target: String,

        #[arg(long, help = "Only set BootNext, without reboot")]
        no_reboot: bool,
    },

    #[command(about = "Scan disk for bootable entries")]
    Scan,
    #[command(about = "Manage boot aliases")]
    Alias {
        #[command(subcommand)]
        action: AliasCommands,
    },
    #[command(about = "Allow non-admin users to run it command")]
    AllowNonAdmin {
        #[arg(long, help = "Username to allow")]
        it_path: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
pub enum AliasCommands {
    #[command(about = "List all boot aliases")]
    List,
    #[command(about = "Add a new boot alias")]
    Add {
        #[arg(help = "Name of the alias")]
        name: String,
        #[arg(help = "Current ID of the boot entry", value_parser = parse_hex)]
        id: u16,
    },
    #[command(about = "Remove an existing boot alias")]
    Remove {
        #[arg(help = "Name of the alias to remove")]
        name: String,
    },
    #[command(about = "Clear all boot aliases")]
    Clear {
        #[arg(short, long, help = "Confirm clearing all aliases")]
        yes: bool,
    },
}

fn parse_hex(s: &str) -> Result<u16, String> {
    u16::from_str_radix(s, 16).map_err(|e| format!("Invalid hex number: {}", e))
}
