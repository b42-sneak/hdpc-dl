use tracing::Level;

mod bypass;
mod cli;
mod constants;
mod data;
mod db;
mod downloader;
mod filters;
mod parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    db::oida().await?;

    Ok(())
}

async fn main2() -> Result<(), anyhow::Error> {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::WARN)
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;

    cli::exec_cli().await.unwrap();

    Ok(())
}
