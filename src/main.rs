// List all to-level modules here
mod cli;
mod core;

/// The main function
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("Copyright 2020 b42-sneak; All rights reserved.\n");

    // Don't put a semicolon on the next line or it won't compile
    cli::handle_commands::exec_cli().await
}
