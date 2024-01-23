use crate::{constants, downloader, filters, jobs::run_jobs_from_path};
use anyhow::Context;
use clap::{Arg, ArgAction, Command};
use tracing::info;

/// Parse and execute the command specified via the CLI
pub async fn exec_cli() -> Result<(), anyhow::Error> {
    info!("Entered cli parsing method");

    // HACK
    let val = &*Box::leak(
        std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            .into_boxed_str(),
    );

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
        .default_value(val)
        .short('d')
        .long("destination"),
      Arg::new("json only")
        .help("Only generate the JSON file")
        .short('j')
        .action(ArgAction::SetTrue)
        .long("json-only"),
      #[cfg(feature = "python_ffi")]
      Arg::new("use bypass")
        .help("Use a Python library to bypass scraping prevention measures")
        .short('b')
        .action(ArgAction::SetTrue)
        .long("use-bypass"),
      Arg::new("get comments")
        .help("Download the comments of all targets (multiple requests)")
        .short('c')
        .action(ArgAction::SetTrue)
        .long("get-comments"),
      Arg::new("v")
        .short('v')
        .action(ArgAction::Count)
        .help("Sets the level of verbosity: 1 for file names, 2 for percentage decimals"),
    ])
    //
    // The big one
    .subcommand(
    Command::new("run-jobs")
      .about("Run a job with db")
      .args(&[Arg::new("jobs-file-location")
        .help("The path to the JSON file")
        .required(true)
      ])
    )
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
          .action(ArgAction::Append)
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
                sub_matches
                    .get_many::<String>("URL")
                    .unwrap()
                    .cloned()
                    .collect::<Vec<_>>(),
                matches.get_one("destination").cloned().unwrap(),
                matches.get_count("v").into(),
                matches.get_flag("json only"),
                false,
                #[cfg(feature = "python_ffi")]
                matches.get_flag("use bypass"),
                matches.get_flag("get comments"),
            )
            .await
        }

        Some("crawl") => {
            let sub_matches = matches.subcommand_matches("crawl").unwrap();

            // Call the crawl function
            downloader::crawl_download(
                sub_matches.get_one("URL").cloned().unwrap(),
                matches.get_one("destination").cloned().unwrap(),
                matches.get_count("v").into(),
                matches.get_flag("json only"),
                (sub_matches.get_one("limit")).cloned().unwrap(),
                (sub_matches.get_one("skip")).cloned().unwrap(),
                sub_matches.get_flag("paging"),
                (sub_matches.get_one("retries")).cloned().unwrap(),
                sub_matches.get_flag("no-download"),
                #[cfg(feature = "python_ffi")]
                matches.get_flag("use bypass"),
                matches.get_flag("get comments"),
            )
            .await
        }

        Some("run-jobs") => {
            let sub_matches = matches.subcommand_matches("run-jobs").unwrap();
            let path: String = sub_matches.get_one("jobs-file-location").cloned().unwrap();
            let verbosity = matches.get_count("v").into();

            run_jobs_from_path(path.into(), verbosity).await
        }

        Some("get-filters") => filters::get_filters().await,

        _ => {
            println!("Something went very wrong");
            Ok(()) // TODO replace with something less ok
        }
    }
}
