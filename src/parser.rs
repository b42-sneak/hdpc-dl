use std::borrow::Cow;

use html_escape::decode_html_entities;
use lazy_static::lazy_static;
use regex::Regex;

use crate::data::{Chapter, Ratings};

lazy_static! {
    static ref CHAPTERS_RX: Regex = Regex::new(r#"<option (?:selected)? data-url="(.+?)">(.+?)</option>"#).unwrap();
    // static ref IMAGE_RX: Regex = Regex::new(r#"<a href="(https://image\.hdporncomics\.com/uploads/.+?\.jpg)" itemprop="contentUrl""#).unwrap();
    static ref IMAGE_RX: Regex = Regex::new(r#"<a href="(https://[^<>" ]+?\.hdporncomics\.com/uploads/.+?\.jpg)"#).unwrap();
    static ref RATINGS_RX: Regex = Regex::new(r#"<span id="upVotes".*?> (\d+) </span>.*?<span id="downVotes".*?> (\d+) </span>.*?<span id="favorite-count".*?> ?(\d+) ?</span>"#).unwrap();
    static ref TITLE_RX: Regex = Regex::new("<title>(.+?) comic porn &ndash; HD Porn Comics</title>").unwrap();
    static ref COMMENTS_RX: Regex = Regex::new(r#"<h3.*?id="comments-title".*?>\s*(.*?)\s*</h3>"#).unwrap();
    static ref POST_ID_RX: Regex = Regex::new(r#"<div id="post-(\d+)"#).unwrap();
}

pub fn extract_chapters(text: &str) -> Vec<Chapter> {
    CHAPTERS_RX
        .captures_iter(text)
        .map(|caps| Chapter {
            name: decode_html_entities(caps.get(1).unwrap().as_str()),
            url: caps.get(2).unwrap().as_str(),
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
