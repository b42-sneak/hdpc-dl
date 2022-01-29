mod cli;
mod constants;
mod downloader;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    cli::exec_cli().await
}
