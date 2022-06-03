use std::borrow::Cow;

use serde::Serialize;

// TODO extract comments using curl 'https://hdporncomics.com/wp-json/api/v1/comments/0123456789/?page_no=1'

#[derive(Debug, Serialize)]
pub struct MetadataV5<'a> {
    pub name: &'a str,
    pub entries: Vec<&'a str>,
}

#[derive(Debug, Serialize)]
/// In case the InfoboxRow cannot be parsed further, its html is stored in `html`
pub struct RawInfoBoxRow<'a> {
    pub name: &'a str,
    pub html: &'a str,
}

#[derive(Debug, Serialize)]
pub enum InfoboxRow<'a> {
    /// In case the InfoboxRow cannot be parsed further, its html is stored in `html`
    Raw(RawInfoBoxRow<'a>),
    /// In case of a tag-like list of tags, the tags are stored in a `Vec` of `TagLike`
    TagLike {
        name: &'a str,
        tags: Vec<TagLike<'a>>,
    },
    /// In case of one `span` element, its contents are stored in `text`
    Text {
        name: &'a str,
        class_name: &'a str,
        text: Option<&'a str>,
    },
}

#[derive(Debug, Serialize)]
pub struct Post<'a> {
    pub name: Cow<'a, str>,
    pub url: &'a str,
}

#[derive(Debug, Serialize)]
pub struct PostBuf {
    pub post_id: u32,
    pub name: String,
    pub url: String,
    pub views: u32,
    pub upvotes: u32,
    pub downvotes: u32,
    pub meta_tags: Vec<String>,
    pub rendered_tags: Vec<TagLikeBuf>,
}

/// A tag-like marker of a post in a search result page
#[derive(Debug, Serialize)]
pub struct TagLikeBuf {
    pub href: String,
    pub text: String,
}

/// A tag-like marker of a post in a search result page
#[derive(Debug, Serialize)]
pub struct TagLike<'a> {
    pub href: &'a str,
    pub text: &'a str,
}

pub struct Ratings {
    pub upvotes: u32,
    pub downvotes: u32,
    pub favorites: u32,
}

#[derive(Debug, Serialize)]
pub struct ResPage<'a> {
    pub url: &'a str,
    pub number: u32,
}

#[derive(Debug, Serialize)]
pub struct CrawlResultV5<'a> {
    /// The version of the JSON document
    pub hdpc_dl_version: i32,

    /// The version of this software used for downloading
    pub program_version: &'static str,

    /// The title of the target
    pub source_url: &'a str,

    /// An RFC 3339 timestamp of the time this was downloaded
    pub download_date: String,

    /// The list of crawled posts
    pub posts: &'a Vec<PostBuf>,
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
    pub metadata: &'a Vec<MetadataV5<'a>>,
    pub picture_urls: &'a Vec<&'a str>,
}

// The data structure for the JSON document to be exported
#[derive(Debug, Serialize)]
pub struct ExportV6<'a> {
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
    pub metadata: &'a Vec<InfoboxRow<'a>>,

    /// A list of chapters (may be empty)
    pub chapters: Vec<Post<'a>>,

    /// The URLs of the individual pictures downloaded from the remote host
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
    pub metadata: &'a Vec<MetadataV5<'a>>,

    /// A list of chapters (may be empty)
    pub chapters: Vec<Post<'a>>,

    /// The URLs of the individual pictures downloaded from the remote host
    pub picture_urls: &'a Vec<&'a str>,
}
