use std::borrow::Cow;

use html_escape::decode_html_entities;
use lazy_static::lazy_static;
use regex::Regex;

use crate::data::{Post, PostBuf, Ratings, ResPage};

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

pub fn extract_target_links(text: String) -> Vec<PostBuf> {
    TARGET_RX
        .captures_iter(&text)
        .map(|caps| PostBuf {
            // TODO try to get rid of the heap allocations
            name: decode_html_entities(caps.get(2).unwrap().as_str()).to_string(),
            url: caps.get(1).unwrap().as_str().to_string(),
        })
        .collect()
}
