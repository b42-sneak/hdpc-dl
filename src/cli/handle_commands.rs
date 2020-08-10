use crate::{constants, core};
use clap::{App, Arg};

/// Parse and execute the command specified via the CLI
pub async fn exec_cli() -> Result<(), anyhow::Error> {
  // This variable is completely useless
  let temp = std::env::current_dir().unwrap();

  // The working directory
  let pwd = temp.to_str().unwrap();

  // Parse the CLI arguments
  let matches = App::new("HDPC downloader")
    .version(constants::VERSION)
    .author("b42-sneak <GitHub @b42-sneak>")
    .about("Downloads comics from HDPC")
    .arg(
      Arg::with_name("URL")
        .help("Sets the URL of the comic to download")
        .required(true)
        .index(1),
    )
    .arg(
      Arg::with_name("destination")
        .help("Sets the download destination path")
        .default_value(pwd)
        .short("d")
        .long("destination"),
    )
    .arg(
      Arg::with_name("v")
        .short("v")
        .multiple(true)
        .help("Sets the level of verbosity"),
    )
    .get_matches();

  // Call the download function
  core::downloader::download_from_url(
    matches.value_of("URL").unwrap(),
    matches.value_of("destination").unwrap(),
    matches.occurrences_of("v"),
  )
  .await
}
