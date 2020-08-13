// List all to-level modules here
mod cli;
mod constants;
mod core;

/// The main function
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Don't put a semicolon on the next line or it won't compile
    cli::handle_commands::exec_cli().await
}
