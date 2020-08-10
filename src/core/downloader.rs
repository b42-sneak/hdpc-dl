use reqwest;
use select::document::Document;
// use select::predicate::{Attr};
use scraper::{Html, Selector};

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

  // println!("{:#?}", res);
  // let document = Document::from_read(res.as_bytes()).unwrap();

  // println!("{:#?}", document);

  // document.find(Attr())

  let document = Html::parse_document(&res);

  let artist_selector =
    Selector::parse("#infoBox > div:nth-child(1) > span.pill-cube > a").unwrap();

  let tags_selector = Selector::parse("#infoBox > div:nth-child(2) > span > a").unwrap();

  let category_selector =
    Selector::parse("#infoBox > div:nth-child(3) > span.pill-cube > a").unwrap();

  let images_selector = Selector::parse("#infoBox > div:nth-child(4) > span.postImages").unwrap();

  let rating_selector = Selector::parse("#infoBox > div:nth-child(6) > span.postLikes").unwrap();

  // TODO Views, likes and dislikes are only available using another POST request

  let date_selector = Selector::parse("#infoBox > div:nth-child(7) > span.postDate").unwrap();

  // This somehow makes this all work
  Ok(())
}
