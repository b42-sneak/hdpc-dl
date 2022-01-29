mod cli;
mod constants;
mod data;
mod downloader;
mod parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    cli::exec_cli().await
}
