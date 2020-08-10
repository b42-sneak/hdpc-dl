// List all to-level modules here
mod cli;
mod constants;
mod core;

/// The main function
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("HDPC Downloader version {}", constants::VERSION);
    println!("Copyright 2020 b42-sneak; All rights reserved.");
    println!("Licensed under the AGPL 3.0 <https://www.gnu.org/licenses/agpl-3.0.en.html>\n");

    // Don't put a semicolon on the next line or it won't compile
    cli::handle_commands::exec_cli().await
}
