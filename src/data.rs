use std::borrow::Cow;

use serde::Serialize;

// TODO extract comments using curl 'https://hdporncomics.com/wp-json/api/v1/comments/0123456789/?page_no=1'

#[derive(Debug, Serialize)]
pub struct Metadata<'a> {
    pub name: &'a str,
    pub entries: Vec<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct Chapter<'a> {
    pub name: Cow<'a, str>,
    pub url: &'a str,
}

pub struct Ratings {
    pub upvotes: u32,
    pub downvotes: u32,
    pub favorites: u32,
}

// The data structure for the JSON document to be exported
// Version 4 (2.2.1)
#[derive(Debug, Serialize)]
pub struct ExportV4<'a> {
    pub hdpc_dl_version: i32,
    pub title: &'a str,
    pub other_title: &'a str,
    pub upvotes: &'a str,
    pub downvotes: &'a str,
    pub favorites: &'a str,
    pub comment_count: &'a str,
    pub download_date: &'a str,
    pub source_url: &'a str,
    pub metadata: &'a Vec<Metadata<'a>>,
    pub picture_urls: &'a Vec<&'a str>,
}

// The data structure for the JSON document to be exported
#[derive(Debug, Serialize)]
pub struct ExportV5<'a> {
    /// The version of the JSON document
    pub hdpc_dl_version: i32,

    /// The version of this software used for downloading
    pub program_version: &'static str,

    /// The id of the downloaded post
    pub post_id: u64,

    /// The title of the target
    pub title: &'a str,

    /// The reported upvote count
    pub upvotes: u32,

    /// THe reported downvote count
    pub downvotes: u32,

    /// The reported favorites count
    /// (Always reports as 0 in the static HTML)
    ///
    /// TODO use another request to get the favorites counter
    pub favorites: u32,

    /// The reported comments count
    pub comment_count: &'a str,

    /// An RFC 3339 timestamp of the time this was downloaded
    pub download_date: String,

    /// The URL of the target described by this document
    pub source_url: &'a str,

    /// A list of key-value(s) pairs provided by the remote host
    pub metadata: &'a Vec<Metadata<'a>>,

    /// A list of chapters (may be empty)
    pub chapters: Vec<Chapter<'a>>,

    /// The URLs of the individual pictures downloaded from the remote host
    pub picture_urls: &'a Vec<&'a str>,
}
