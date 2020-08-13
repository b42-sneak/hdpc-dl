use crate::{constants, core};
use clap::{App, AppSettings, Arg, SubCommand};

/// Parse and execute the command specified via the CLI
pub async fn exec_cli() -> Result<(), anyhow::Error> {
  // This variable is completely useless
  let temp = std::env::current_dir().unwrap();

  // The working directory
  let pwd = temp.to_str().unwrap();

  // The app definition
  let app = App::new(constants::NAME)
    .version(constants::VERSION)
    .author("b42-sneak <GitHub @b42-sneak>")
    .about("Downloads comics from HDPC")
    .after_help(constants::LICENSE)
    .settings(&[
      AppSettings::SubcommandRequiredElseHelp,
      AppSettings::DisableHelpSubcommand,
      AppSettings::GlobalVersion,
      AppSettings::VersionlessSubcommands,
    ])
    .arg(
      Arg::with_name("destination")
        .help("Sets the download destination path")
        .default_value(pwd)
        .short("d")
        .long("destination"),
    )
    .arg(
      Arg::with_name("json only")
        .help("Only generate the JSON file")
        .short("j")
        .long("json-only"),
    )
    .arg(
      Arg::with_name("v")
        .short("v")
        .multiple(true)
        .help("Sets the level of verbosity: 1 for file names, 2 for percentage decimals"),
    )
    .subcommand(
      SubCommand::with_name("one")
        .about("Downloads one comic")
        .after_help(constants::LICENSE)
        .arg(
          Arg::with_name("URL")
            .help("Sets the URL of the comic to download")
            .required(true)
            .index(1),
        ),
    )
    .subcommand(
      SubCommand::with_name("crawl")
        .about("Finds all comics on a URL and downloads them all")
        .after_help(constants::LICENSE),
    );

  // Parse the CLI arguments
  let matches = app.get_matches();

  println!("{} {}", constants::NAME, constants::VERSION);
  println!("{}\n", constants::LICENSE);

  match matches.subcommand_name() {
    Some("one") => {
      let sub_matches = matches.subcommand_matches("one").unwrap();

      // Call the download function
      core::downloader::download_from_url(
        sub_matches.value_of("URL").unwrap(),
        matches.value_of("destination").unwrap(),
        matches.occurrences_of("v"),
        matches.is_present("json only"),
      )
      .await
    }
    _ => {
      // TODO implement the crawler
      println!("To be implemented (TODO for me)");

      Ok(())
    }
  }
}
