use crate::{constants, downloader, filters};
use clap::{Arg, Command};

/// Parse and execute the command specified via the CLI
pub async fn exec_cli() -> Result<(), anyhow::Error> {
    // This variable is completely useless
    let temp = std::env::current_dir().unwrap();

    // The working directory
    let pwd = temp.to_str().unwrap();

    // The app definition
    let app = Command::new(constants::NAME)
    .version(constants::VERSION)
    .author("b42-sneak <GitHub @b42-sneak>")
    .about("Downloads comics from HDPC")
    .after_help(constants::LICENSE)
    .subcommand_required(true)
    .disable_help_subcommand(true)
    .propagate_version(true)
    //
    // Main args
    .args(&[
      Arg::new("destination")
        .help("Sets the download destination path")
        .default_value(pwd)
        .short('d')
        .long("destination"),
      Arg::new("json only")
        .help("Only generate the JSON file")
        .short('j')
        .long("json-only"),
      Arg::new("v")
        .short('v')
        .multiple_occurrences(true)
        .help("Sets the level of verbosity: 1 for file names, 2 for percentage decimals"),
    ])
    //
    // One
    .subcommand(
      Command::new("get")
        .alias("one")
        .about("Downloads one comic")
        .after_help(constants::LICENSE)
        .args(&[Arg::new("URL")
          .help("Sets the URL of the comic to download")
          .required(true)
          .multiple_values(true)
          .index(1)]),
    )
    //
    // Crawl
    .subcommand(
      Command::new("crawl")
        .about("Finds all comics on a URL and downloads them all")
        .after_help(constants::LICENSE)
        .args(&[
          Arg::new("URL")
            .help("Sets the URL of the page to be crawled")
            .required(true)
            .index(1),
          Arg::new("limit")
            .help("Limit to n finding(s) to be downloaded")
            .short('l')
            .long("limit")
            .default_value("0"),
          Arg::new("skip")
            .help("Skip the first n finding(s)")
            .short('s')
            .long("skip")
            .default_value("0"),
          Arg::new("retries")
            .help("How often to retry if a download fails")
            .short('r')
            .long("retries")
            .default_value("0"),
            Arg::new("paging")
            .help("Tries to continue on the next page withing the download limit & offset")
            .short('p')
            .long("paging"),
            Arg::new("no-download")
            .help("Exports the crawl result without downloading anything else")
            .short('n')
            .long("no-download")
        ]),
    )
    .subcommand(
      Command::new("get-filters")
        .about("Downloads the filter api data and stores it in JSON files")
        .after_help(constants::LICENSE),
    );

    // Parse the CLI arguments
    let matches = app.get_matches();

    println!("{} {}", constants::NAME, constants::VERSION);
    println!("{}\n", constants::LICENSE);

    match matches.subcommand_name() {
        Some("get") => {
            let sub_matches = matches.subcommand_matches("get").unwrap();

            // Call the download function
            downloader::download_from_urls(
                sub_matches.values_of("URL").unwrap().collect::<Vec<_>>(),
                matches.value_of("destination").unwrap(),
                matches.occurrences_of("v"),
                matches.is_present("json only"),
                false,
            )
            .await
        }

        Some("crawl") => {
            let sub_matches = matches.subcommand_matches("crawl").unwrap();

            // Call the crawl function
            downloader::crawl_download(
                sub_matches.value_of("URL").unwrap(),
                matches.value_of("destination").unwrap(),
                matches.occurrences_of("v"),
                matches.is_present("json only"),
                (sub_matches.value_of_t("limit")).unwrap_or_else(|e| e.exit()),
                (sub_matches.value_of_t("skip")).unwrap_or_else(|e| e.exit()),
                sub_matches.is_present("paging"),
                (sub_matches.value_of_t("retries")).unwrap_or_else(|e| e.exit()),
                sub_matches.is_present("no-download"),
            )
            .await
        }

        Some("get-filters") => filters::get_filters().await,

        _ => {
            println!("Something went very wrong");
            Ok(()) // TODO replace with something less ok
        }
    }
}
