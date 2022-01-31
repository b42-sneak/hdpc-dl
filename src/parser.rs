use std::borrow::Cow;

use html_escape::decode_html_entities;
use lazy_static::lazy_static;
use regex::Regex;
use str_overlap::Overlap;

use crate::data::{Post, PostBuf, Ratings, ResPage, TagLike};

lazy_static! {
    static ref CHAPTERS_RX: Regex = Regex::new(r#"<option (?:selected)? data-url="(.+?)">(.+?)</option>"#).unwrap();
    // static ref IMAGE_RX: Regex = Regex::new(r#"<a href="(https://image\.hdporncomics\.com/uploads/.+?\.jpg)" itemprop="contentUrl""#).unwrap();
    static ref IMAGE_RX: Regex = Regex::new(r#"<a href="(https://[^<>" ]+?\.hdporncomics\.com/uploads/.+?\.jpg)"#).unwrap();
    static ref RATINGS_RX: Regex = Regex::new(r#"<span id="upVotes".*?> (\d+) </span>.*?<span id="downVotes".*?> (\d+) </span>.*?<span id="favorite-count".*?> ?(\d+) ?</span>"#).unwrap();
    static ref TITLE_RX: Regex = Regex::new("<title>(.+?) comic porn &ndash; HD Porn Comics</title>").unwrap();
    static ref COMMENTS_RX: Regex = Regex::new(r#"<h3.*?id="comments-title".*?>\s*(.*?)\s*</h3>"#).unwrap();
    static ref POST_ID_RX: Regex = Regex::new(r#"<div id="post-(\d+)"#).unwrap();
    static ref RES_PAGES_RX: Regex = Regex::new(r#"<option data-url="(.+?)"(?: selected )?>(\d+)</option>"#).unwrap();
    static ref TARGET_RX: Regex = Regex::new(include_str!("./regex/target.rx")).unwrap();
    static ref TAG_LIKE_RX: Regex = Regex::new(r#"<a href="([^"]+)" rel="tag">([^<]+)</a>"#).unwrap();
}

pub fn extract_chapters(text: &str) -> Vec<Post> {
    CHAPTERS_RX
        .captures_iter(text)
        .map(|caps| Post {
            name: decode_html_entities(caps.get(2).unwrap().as_str()),
            url: caps.get(1).unwrap().as_str(),
        })
        .collect()
}

pub fn extract_image_urls(text: &str) -> Vec<&str> {
    IMAGE_RX
        .captures_iter(text)
        .map(|caps| caps.get(1).unwrap().as_str())
        .collect()
}

pub fn extract_ratings(text: &str) -> Option<Ratings> {
    let caps = RATINGS_RX.captures_iter(text).next()?;

    // .expect is used because the string above matched a known regex pattern
    const MSG: &str = "Couldn't parse ratings. Are the integers valid?";
    Some(Ratings {
        upvotes: caps.get(1).unwrap().as_str().parse().expect(MSG),
        downvotes: caps.get(2).unwrap().as_str().parse().expect(MSG),
        favorites: caps.get(3).unwrap().as_str().parse().expect(MSG),
    })
}

pub fn extract_title(text: &str) -> Option<Cow<str>> {
    Some(decode_html_entities(
        TITLE_RX.captures_iter(text).next()?.get(1)?.as_str(),
    ))
}

pub fn extract_comment_count(text: &str) -> Option<&str> {
    Some(COMMENTS_RX.captures_iter(text).next()?.get(1)?.as_str())
}

pub fn extract_post_id(text: &str) -> Option<u64> {
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
                url: caps.get(1).unwrap().as_str().to_string(),
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
                    .map(|tag| TagLike {
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
