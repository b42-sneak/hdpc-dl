use reqwest;

/// Downloads a comic given a URL and a destination
pub async fn download_from_url(
  url: &str,
  dest: &str,
  _verbosity: u64,
) -> Result<(), anyhow::Error> {
  println!("Destination: {}", dest);
  println!("URL: {}", url);

  let resp = reqwest::get(url).await?.text().await?;
  println!("{:#?}", resp);

  // This somehow makes this all work
  Ok(())
}
