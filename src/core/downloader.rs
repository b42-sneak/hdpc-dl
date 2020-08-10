use chrono::prelude::*;
use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;

/// Downloads a comic given a URL and a destination
pub async fn download_from_url(
  url: &str,
  dest: &str,
  _verbosity: u64,
) -> Result<(), anyhow::Error> {
  // Inform the user the actions
  println!("Destination: {}", dest);
  println!("URL: {}", url);

  let client = reqwest::Client::new();

  let res = client.get(url).send().await?.text().await?.to_string();

  let document = Html::parse_document(&res);

  let mut artists: Vec<&str> = Vec::new();
  let mut tags: Vec<&str> = Vec::new();
  let mut categories: Vec<&str> = Vec::new();
  let mut images: &str = "";
  let mut rating: &str = "";
  let mut upload_date: &str = "";
  let mut title: &str = "";

  let mut picture_urls: Vec<&str> = Vec::new();

  let artist_selector =
    Selector::parse("#infoBox > div:nth-child(1) > span.pill-cube > a").unwrap();

  let tags_selector = Selector::parse("#infoBox > div:nth-child(2) > span > a").unwrap();

  let category_selector =
    Selector::parse("#infoBox > div:nth-child(3) > span.pill-cube > a").unwrap();

  let images_selector = Selector::parse("#infoBox > div:nth-child(4) > span.postImages").unwrap();

  let rating_selector = Selector::parse("#infoBox > div:nth-child(6) > span.postLikes").unwrap();

  // TODO Views, likes and dislikes are only available using another POST request

  let date_selector = Selector::parse("#infoBox > div:nth-child(7) > span.postDate").unwrap();

  let title_selector = Selector::parse("article.postContent.text-white > h2").unwrap();

  for element in document.select(&artist_selector) {
    artists.push(element.text().next().unwrap());
  }

  for element in document.select(&tags_selector) {
    tags.push(element.text().next().unwrap());
  }

  for element in document.select(&category_selector) {
    categories.push(element.text().next().unwrap());
  }

  for element in document.select(&images_selector) {
    images = element.text().next().unwrap();
    images = images.trim();
  }

  for element in document.select(&rating_selector) {
    rating = element.text().next().unwrap();
    rating = rating.trim();
  }

  for element in document.select(&date_selector) {
    upload_date = element.text().skip(1).next().unwrap();
    upload_date = upload_date.trim();
  }

  for element in document.select(&title_selector) {
    title = element.text().next().unwrap();
    title = title.trim();
  }

  let picture_selector =
    Selector::parse("article.postContent.text-white > div > figure > a").unwrap();

  for element in document.select(&picture_selector) {
    picture_urls.push(
      element
        .value()
        .attrs()
        .find(|attr| attr.0 == "href")
        .unwrap()
        .1,
    );
  }

  let data = Export {
    title: title,
    artists: artists,
    tags: tags,
    categories: categories,
    images: images,
    rating: rating,
    upload_date: upload_date,
    download_date: &Utc::now().to_rfc3339(),
    picture_urls: picture_urls,
  };

  let serialized = serde_json::to_string_pretty(&data).unwrap();

  // Build-a-path
  let mut path = dest.to_owned() + "/" + title;

  // Create the destination folder if it doesn't exist
  std::fs::create_dir_all(std::path::Path::new(&path))
    .expect("Failed to create directory.\nTry to specify another path.\n");

  // The JSON path
  path.push_str("/hdpc-info.json");

  // Write the JSON file to disk
  std::fs::write(&path, serialized)
    .expect("Failed to create the JSON file.\nTry to specify another path.\n");

  // This somehow makes this all work
  Ok(())
}

#[derive(Debug, Serialize)]
struct Export<'a> {
  title: &'a str,
  download_date: &'a str,
  artists: Vec<&'a str>,
  tags: Vec<&'a str>,
  categories: Vec<&'a str>,
  images: &'a str,
  rating: &'a str,
  upload_date: &'a str,
  picture_urls: Vec<&'a str>,
}
