mod bypass;
mod cli;
mod constants;
mod data;
mod downloader;
mod filters;
mod parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    cli::exec_cli().await
}
