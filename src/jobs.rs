use std::{fs::read_to_string, path::PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

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
}

pub async fn run_jobs_from_path(path: PathBuf, verbosity: u8) -> anyhow::Result<()> {
    log::info!("Performing jobs from {}", path.to_string_lossy());

    let jobs = read_to_string(&path).context("Reading the jobs file failed")?;
    let jobs: Vec<Job> = serde_json::from_str(&jobs).context("Parsing the jobs file failed")?;

    Ok(())
}
