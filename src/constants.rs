/// The version (semver) of this application as specified in Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const LICENSE: &str = concat![
    "Copyright 2020-2024 b42-sneak; All rights reserved.\n",
    "Licensed under the AGPL 3.0 <https://www.gnu.org/licenses/agpl-3.0.en.html>"
];

pub const NAME: &str = "HDPC Downloader";

/// The number of targets per (full) results page
pub const TARGETS_PER_PAGE: usize = 21;

pub const API_FILTER_PATH: &str = "https://hdporncomics.com/?rest_route=/api/v1/filter";
