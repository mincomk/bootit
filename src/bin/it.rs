use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let status = match Command::new("bootit").arg("boot").args(&args).status() {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Error: Failed to execute 'bootit' command: {}", e);
            eprintln!("Make sure 'bootit' is installed and in your PATH.");

            #[cfg(windows)]
            eprintln!(
                "\nIf you are not running as Administrator, try opening an elevated terminal."
            );

            std::process::exit(127);
        }
    };

    std::process::exit(status.code().unwrap_or(1));
}
