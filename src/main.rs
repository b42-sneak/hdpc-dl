use tracing::Level;

#[cfg(feature = "python_ffi")]
mod bypass;
mod constants;
mod data;
mod db;
mod downloader;
mod filters;
mod jobs;
mod old_cli;
mod parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::WARN)
        .finish();

    let db_client = db::connect().await?;

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;

    old_cli::exec_cli().await?;

    Ok(())
}
