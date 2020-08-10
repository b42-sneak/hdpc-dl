use reqwest;
use select::document::Document;

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
  let document = Document::from_read(res.as_bytes());

  println!("{:#?}", document);
  // This somehow makes this all work
  Ok(())
}
