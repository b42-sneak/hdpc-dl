use std::borrow::Cow;

use html_escape::decode_html_entities;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;
use str_overlap::Overlap;
use tracing::{info, info_span};

use crate::data::{
    ApiViewResponse, Comment, Comments, InfoboxRow, Post, PostBuf, RawInfoBoxRow, ResPage, TagLike,
    TagLikeBuf,
};

// Artist
// Group
// Upload date

lazy_static! {
    static ref CHAPTERS_RX: Regex = Regex::new(r#"<option (?:selected)? data-url="(.+?)">(.+?)</option>"#).unwrap();
    // static ref IMAGE_RX: Regex = Regex::new(r#"<a href="(https://image\.hdporncomics\.com/uploads/.+?\.jpg)" itemprop="contentUrl""#).unwrap();
    static ref IMAGE_RX: Regex = Regex::new(r#"<a href="(https://[^<>" ]+?\.hdporncomics\.com/uploads/.+?\.jpg)"#).unwrap();
    // static ref RATINGS_RX_FULL: Regex = Regex::new(r#"<span id="upVotes".*?> (\d+) </span>.*?<span id="downVotes".*?> (\d+) </span>.*?<span id="favorite-count".*?> ?(\d+) ?</span>"#).unwrap();
    static ref RATINGS_RX: Regex = Regex::new(r#"<span id="upVotes"[^>]*>\s*(\d+)\s*</span>.*?<span id="downVotes"[^>]*>\s*(\d+)\s*</span>"#).unwrap();
    static ref TITLE_RX: Regex = Regex::new("<title>(.+?) (?:comic porn )?(?:&ndash;|-) HD Porn Comics</title>").unwrap();
    static ref COMMENTS_RX: Regex = Regex::new(r#"<h3.*?id="comments-title".*?>\s*(.*?)\s*</h3>"#).unwrap();
    static ref POST_ID_RX: Regex = Regex::new(r#"<div id="post-(\d+)"#).unwrap();
    static ref RES_PAGES_RX: Regex = Regex::new(r#"<option data-url="(.+?)"(?: selected )?>(\d+)</option>"#).unwrap();
    static ref TARGET_RX: Regex = Regex::new(include_str!("./regex/target.rx")).unwrap();
    static ref TAG_LIKE_RX: Regex = Regex::new(r#"<span class="scrolltaxonomy-item"><a href="([^"]+)" rel="tag">([^<]+)</a></span>"#).unwrap();
    static ref INFOBOX_LINE_RX: Regex = Regex::new(r#"<div class="flex items-center"> <span class="text-gray-400 whitespace-nowrap">(.+?) : ?</span> (.+?)</div>"#).unwrap();
    static ref INFOBOX_TEXT_RX: Regex = Regex::new(r#"<span class="ml-4 ([^"]+)"> ?(.*?) ?</span>"#).unwrap();
}

pub fn extract_from_infobox_row(row: RawInfoBoxRow) -> InfoboxRow {
    let tag_like_vec = TAG_LIKE_RX
        .captures_iter(&row.html)
        .map(|caps| TagLike {
            href: caps.get(1).unwrap().as_str(),
            text: caps.get(2).unwrap().as_str(),
        })
        .collect::<Vec<_>>();

    if !tag_like_vec.is_empty() {
        return InfoboxRow::TagLike {
            name: row.name,
            tags: tag_like_vec,
        };
    }

    if let Some(caps) = INFOBOX_TEXT_RX.captures_iter(row.html).next() {
        return InfoboxRow::Text {
            name: row.name,
            class_name: caps.get(1).unwrap().as_str(),
            text: caps.get(2).map(|text| text.as_str()),
        };
    }

    InfoboxRow::Raw(row)
}

pub fn extract_info_box_rows(text: &str) -> Vec<RawInfoBoxRow> {
    info!("Extracting info box rows");

    INFOBOX_LINE_RX
        .captures_iter(text)
        .map(|caps| RawInfoBoxRow {
            name: caps.get(1).unwrap().as_str(),
            html: caps.get(2).unwrap().as_str(),
        })
        .collect()
}

pub fn extract_chapters(text: &str) -> Vec<Post> {
    info!("Extracting chapters");

    CHAPTERS_RX
        .captures_iter(text)
        .map(|caps| Post {
            name: decode_html_entities(caps.get(2).unwrap().as_str()),
            url: caps.get(1).unwrap().as_str(),
        })
        .collect()
}

pub fn extract_image_urls(text: &str) -> Vec<&str> {
    info!("Extracting image URLs");

    IMAGE_RX
        .captures_iter(text)
        .map(|caps| caps.get(1).unwrap().as_str())
        .collect()
}

/// Downloads views, likes, dislikes, favourites, and the post id from the API
///
/// The `url` must be the exact post url, as it's set in the `Referer` header to select the desired post
pub async fn get_api_view(http_client: &Client, url: &str) -> reqwest::Result<ApiViewResponse> {
    info!("Getting API info from URL {url}");
    Ok(http_client
        .get("https://hdporncomics.com/?rest_route=%2Fapi%2Fv1%2Fview")
        .header("Referer", url)
        .send()
        .await?
        .json()
        .await
        .unwrap())
}

pub async fn get_comments(post_id: u64, client: &Client) -> reqwest::Result<Vec<Comment>> {
    info_span!("Downloading comments");

    let res: Comments = client
        .get(format!(
            "https://hdporncomics.com/wp-json/api/v1/comments/{post_id}?page_no=1"
        ))
        .send()
        .await?
        .json()
        .await?;

    info!("Got comments page 1");

    let mut comments = res.comments;

    for i in 2..=res.total_pages {
        let mut res: Comments = client
            .get(format!(
                "https://hdporncomics.com/wp-json/api/v1/comments/{post_id}?page_no={i}"
            ))
            .send()
            .await?
            .json()
            .await?;

        info!("Got comments page {i}");
        comments.append(&mut res.comments);
    }

    Ok(comments)
}

pub fn extract_title(text: &str) -> Option<Cow<str>> {
    info!("Extracting post title");

    Some(decode_html_entities(
        TITLE_RX.captures_iter(text).next()?.get(1)?.as_str(),
    ))
}

pub fn extract_comment_count(text: &str) -> Option<&str> {
    info!("Extracting comment count");

    Some(COMMENTS_RX.captures_iter(text).next()?.get(1)?.as_str())
}

pub fn extract_post_id(text: &str) -> Option<u64> {
    info!("Extracting post id");

    POST_ID_RX
        .captures_iter(text)
        .next()?
        .get(1)?
        .as_str()
        .parse()
        .ok()
}

/// Extracts the links and numbers of the result pages
pub fn extract_res_page_links(text: &str) -> Vec<ResPage> {
    RES_PAGES_RX
        .captures_iter(text)
        .map(|page| ResPage {
            url: page.get(1).unwrap().as_str(),
            number: page.get(2).unwrap().as_str().parse().unwrap(),
        })
        .collect()
}

/// Converts an HTML-response String into useful data using a lot of regex-matching
pub fn extract_target_links(text: String) -> Vec<PostBuf> {
    TARGET_RX
        .captures_iter(&text)
        .map(|caps| {
            let name = caps
                .name("title_main")
                .unwrap()
                .as_str()
                .overlap_start(caps.name("title_alt").unwrap().as_str());

            let get_num = |name| {
                caps.name(name)
                    .unwrap()
                    .as_str()
                    .parse_with_suffix()
                    .unwrap()
            };

            PostBuf {
                // TODO try to get rid of the heap allocations
                post_id: caps.name("post_id").unwrap().as_str().parse().unwrap(),
                name: decode_html_entities(name).to_string(),
                url: caps.name("url").unwrap().as_str().to_string(),
                views: get_num("views"),
                upvotes: get_num("upvotes"),
                downvotes: get_num("downvotes"),
                meta_tags: caps
                    .name("tags")
                    .unwrap()
                    .as_str()
                    .to_string()
                    .split_ascii_whitespace()
                    .map(|s| s.to_string()) // TODO don't allocate like mad
                    .collect(),
                rendered_tags: TAG_LIKE_RX
                    .captures_iter(caps.get(0).unwrap().as_str())
                    .map(|tag| TagLikeBuf {
                        href: tag.get(1).unwrap().as_str().to_string(),
                        text: tag.get(2).unwrap().as_str().to_string(),
                    })
                    .collect(),
            }
        })
        .collect()
}

trait SuffixParse {
    fn parse_with_suffix(&self) -> anyhow::Result<u32>;
}

impl SuffixParse for &str {
    fn parse_with_suffix(&self) -> anyhow::Result<u32> {
        let unwrap_wrap = if self.ends_with("k") || self.ends_with("K") {
            self[..self.len() - 1]
                .parse()
                .and_then(|num: f32| Ok((num * 1_000.0) as u32))?
        } else if self.ends_with("m") || self.ends_with("M") {
            self[..self.len() - 1]
                .parse()
                .and_then(|num: f32| Ok((num * 1_000_000.0) as u32))?
        } else {
            self.parse()?
        };

        // Unwrap the result and wrap again, because... (type-)reasons?
        Ok(unwrap_wrap)
    }
}
