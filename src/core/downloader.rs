use reqwest;
use tokio;

/// Downloads a comic given a URL and a destination
pub fn download_from_url(url: &str, dest: &str, _verbosity: u64) -> () {
  println!("Destination: {}", dest);
  println!("URL: {}", url);

  let mut rt = tokio::runtime::Runtime::new().unwrap();
  let res = rt.block_on(reqwest::get(url));

  println!("{:?}", res);
}
