use chrono::prelude::*;
use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;
use tokio::{fs, io::AsyncWriteExt};

/// Downloads a comic given a URL and a destination
pub async fn download_from_url(
  url: &str,
  dest: &str,
  _verbosity: u64,
) -> Result<(), anyhow::Error> {
  // Inform the user about the actions to be taken
  println!("Destination: {}", dest);
  println!("URL: {}", url);

  // Create a client to make requests with
  let client = reqwest::Client::new();

  // Request the HTML file from the server
  let res = client.get(url).send().await?.text().await?.to_string();

  // Parse the HTML from the response
  let document = Html::parse_document(&res);

  // The URLs of the pictures to be downloaded
  let mut picture_urls: Vec<&str> = Vec::new();

  // The metadata to be extracted
  let mut metadata: Vec<Metadata> = Vec::new();

  // The selectors for the strings to be extracted
  let row_selector = Selector::parse("#infoBox > div.pt-1").unwrap();
  let span_selector = Selector::parse("span").unwrap();

  // The selector for the image URLs
  let picture_selector =
    Selector::parse("article.postContent.text-white > div > figure > a").unwrap();

  // TODO Views, likes and dislikes are only available using another POST request

  // Extract the title
  let title = match document
    .select(&Selector::parse("article.postContent.text-white > h2").unwrap())
    .next()
  {
    Some(element) => element.text().next().unwrap_or("no-title-found").trim(),
    None => "Could-not-extract-title",
  };

  // Extract all metadata
  for row in document.select(&row_selector) {
    let mut columns = row.select(&span_selector);

    let name = match columns.next() {
      Some(content) => remove_colon(content.text().next().unwrap_or("no-text-here").trim()),
      None => "nothing-to-be-seen-here",
    };

    if name == "*" {
      continue;
    };

    let mut entries = Vec::new();

    for column in columns {
      for content in column.text() {
        if content.trim() != "" {
          entries.push(content.trim());
        }
      }
    }

    metadata.push(Metadata { name, entries });
  }

  // Loop over all imagesas
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

  // Fill the data structure for the JSON document to be exported
  let data = Export {
    hdpc_dl_version: 2,
    title: title,
    download_date: &Utc::now().to_rfc3339(),
    metadata: &metadata,
    picture_urls: &picture_urls,
  };

  // Serialize the data to JSON
  let serialized = serde_json::to_string_pretty(&data).unwrap();

  // Build-a-path
  let path = dest.to_owned() + "/" + title;

  // Create the destination folder if it doesn't exist
  std::fs::create_dir_all(std::path::Path::new(&path))
    .expect("Failed to create directory.\nTry to specify another path.\n");

  // The JSON path
  let json_path = path.clone() + "/hdpc-info.json";

  // Write the JSON file to disk
  std::fs::write(&json_path, serialized)
    .expect("Failed to create the JSON file.\nTry to specify another path.\n");

  // Log successful JSON file creation
  println!("Created JSON file at \"{}\"\n", &json_path);

  // Download the images and write them to disk
  for i in 0..picture_urls.len() {
    // Request the image from the server
    let req = client.get(picture_urls[i]).send();

    // Generate a file name
    let file_name = format!(
      "{:03}-{}",
      i + 1,
      reqwest::Url::parse(picture_urls[i])
        .unwrap()
        .path_segments()
        .and_then(std::iter::Iterator::last)
        .unwrap()
    );

    // Make a file path for Tokio
    let file_path_str = path.clone() + "/" + &file_name;
    let file_path = std::path::Path::new(&file_path_str);

    // Await the response from the server
    let mut res = req.await?;

    // Make Tokio open the (new) file
    let mut image_file = fs::OpenOptions::new()
      .create(true)
      .append(true)
      .open(&file_path)
      .await?;

    // Write the file to disk
    while let Some(chunk) = res.chunk().await? {
      image_file.write_all(&chunk).await?;
    }

    println!(
      "Wrote file {:03}/{:03}: {}",
      i + 1,
      picture_urls.len(),
      file_name
    )
  }

  println!(
    "\nSuccessfully downloaded all {} images from \"{}\".",
    picture_urls.len(),
    title
  );

  // This somehow makes this all work
  Ok(())
}

// The data structure for the JSON document to be exported
#[derive(Debug, Serialize)]
struct Export<'a> {
  hdpc_dl_version: i32,
  title: &'a str,
  download_date: &'a str,
  metadata: &'a Vec<Metadata<'a>>,
  picture_urls: &'a Vec<&'a str>,
}

#[derive(Debug, Serialize)]
struct Metadata<'a> {
  name: &'a str,
  entries: Vec<&'a str>,
}

/// Removes the space and the colon from the end of a string slice
fn remove_colon(s: &str) -> &str {
  if s.len() < 2 || s[s.len() - 2..] != *" :" {
    s
  } else {
    &s[..s.len() - 2]
  }
}
