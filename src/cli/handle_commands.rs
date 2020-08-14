use crate::{constants, core};
use clap::{value_t, App, AppSettings, Arg, SubCommand};

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
    //
    // Main args
    .args(&[
      Arg::with_name("destination")
        .help("Sets the download destination path")
        .default_value(pwd)
        .short("d")
        .long("destination"),
      Arg::with_name("json only")
        .help("Only generate the JSON file")
        .short("j")
        .long("json-only"),
      Arg::with_name("v")
        .short("v")
        .multiple(true)
        .help("Sets the level of verbosity: 1 for file names, 2 for percentage decimals"),
    ])
    //
    // One
    .subcommand(
      SubCommand::with_name("one")
        .about("Downloads one comic")
        .after_help(constants::LICENSE)
        .args(&[Arg::with_name("URL")
          .help("Sets the URL of the comic to download")
          .required(true)
          .index(1)]),
    )
    //
    // Crawl
    .subcommand(
      SubCommand::with_name("crawl")
        .about("Finds all comics on a URL and downloads them all")
        .after_help(constants::LICENSE)
        .args(&[
          Arg::with_name("URL")
            .help("Sets the URL of the page to be crawled")
            .required(true)
            .index(1),
          Arg::with_name("limit")
            .help("Limit to n finding(s) to be downloaded")
            .short("l")
            .long("limit")
            .default_value("0"),
          Arg::with_name("skip")
            .help("Skip the first n finding(s)")
            .short("s")
            .long("skip")
            .default_value("0"),
          Arg::with_name("paging")
            .help("Tries to continue on the next page withing the download limit & offset")
            .short("p")
            .long("paging"),
        ]),
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

    Some("crawl") => {
      let sub_matches = matches.subcommand_matches("crawl").unwrap();

      // Call the crawl function
      core::downloader::crawl_download(
        sub_matches.value_of("URL").unwrap(),
        matches.value_of("destination").unwrap(),
        matches.occurrences_of("v"),
        matches.is_present("json only"),
        value_t!(sub_matches.value_of("limit"), usize).unwrap_or_else(|e| e.exit()),
        value_t!(sub_matches.value_of("skip"), usize).unwrap_or_else(|e| e.exit()),
        sub_matches.is_present("paging"),
      )
      .await
    }

    _ => {
      println!("Something went very wrong");
      Ok(()) // TODO replace with something less ok
    }
  }
}
