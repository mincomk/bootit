use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bootit")]
#[command(about = "Simple in-system boot selection utility", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(
        long,
        help = "Path to configuration file",
        default_value = "/boot/bootit.yaml"
    )]
    pub config_path: PathBuf,
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
}

#[derive(Subcommand)]
pub enum AliasCommands {
    #[command(about = "List all boot aliases")]
    List,
    #[command(about = "Add a new boot alias")]
    Add {
        #[arg(help = "Name of the alias")]
        name: String,
        #[arg(help = "Current ID of the boot entry")]
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
