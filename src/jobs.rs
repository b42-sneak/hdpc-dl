use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::filters::{Filter, FilterValue};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Job {
    /// Gets all global filter lists (artist, category, characters, groups, tags, and parody)
    FetchFilters,

    /// Downloads one comic from a URL
    DownloadOne { url: String, destination: PathBuf },

    /// Crawls a URL for comics without downloading them
    CrawlUrlIndex { url: String },

    /// Implies
    ///
    /// - [`Job::CrawlUrlIndex`] once
    /// - [`Job::DownloadOne`] on all results
    CrawlUrlDownload { url: String, destination: PathBuf },

    /// Import data from older versions of this software
    ImportFromJson {
        artist_path: Option<PathBuf>,
        category_path: Option<PathBuf>,
        characters_path: Option<PathBuf>,
        groups_path: Option<PathBuf>,
        tags_path: Option<PathBuf>,
        parody_path: Option<PathBuf>,
    },
}

pub async fn run_jobs_from_path(path: PathBuf, verbosity: u8) -> anyhow::Result<()> {
    log::info!("Performing jobs from {}", path.to_string_lossy());

    let jobs = read_to_string(&path).context("Reading the jobs file failed")?;
    let jobs: Vec<Job> = serde_json::from_str(&jobs).context("Parsing the jobs file failed")?;

    for job in jobs {
        match job {
            Job::FetchFilters => todo!(),
            Job::DownloadOne { url, destination } => todo!(),
            Job::CrawlUrlIndex { url } => todo!(),
            Job::CrawlUrlDownload { url, destination } => todo!(),
            Job::ImportFromJson {
                artist_path,
                category_path,
                characters_path,
                groups_path,
                tags_path,
                parody_path,
            } => {
                if let Some(artists) = artist_path {
                    let artists =
                        read_to_string(&artists).context("Reading the artists file failed")?;
                    let artists: HashMap<u16, FilterValue> =
                        serde_json::from_str(&artists).context("Parsing the jobs file failed")?;
                };
            }
        }
    }

    Ok(())
}
