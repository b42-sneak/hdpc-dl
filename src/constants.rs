/// The version (semver) of this application as specified in Cargo.toml
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub const LICENSE: &'static str = concat![
    "Copyright 2020 b42-sneak; All rights reserved.\n",
    "Licensed under the AGPL 3.0 <https://www.gnu.org/licenses/agpl-3.0.en.html>"
];

pub const NAME: &'static str = "HDPC Downloader";
