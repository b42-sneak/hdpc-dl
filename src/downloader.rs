use crate::{
    constants,
    data::*,
    parser::{
        extract_chapters, extract_comment_count, extract_image_urls, extract_post_id,
        extract_ratings, extract_res_page_links, extract_target_links, extract_title,
    },
};
use chrono::prelude::*;
use scraper::{Html, Selector};
use tokio::{fs, io::AsyncWriteExt};

/// Downloads a comic given a URL and a destination
pub async fn download_from_url(
    url: &str,
    dest: &str,
    verbosity: u64,
    json_only: bool,
    use_padding: bool,
) -> Result<(), anyhow::Error> {
    let padding = if use_padding { "  " } else { "" };

    // Inform the user about the actions to be taken
    println!("{padding}Destination: {}", dest);
    println!("{padding}URL: {}", url);

    // Create a client to make requests with
    let client = reqwest::Client::new();

    // Request the HTML file from the server
    let text = client.get(url).send().await?.text().await?.to_string();

    // Parse the HTML from the response
    // TODO remove this (and the entire HTML parsing process as well)
    let document = Html::parse_document(&text);

    // The URLs of the pictures to be downloaded
    let picture_urls = extract_image_urls(&text);

    // The metadata to be extracted
    let mut metadata: Vec<Metadata> = Vec::new();

    // The selectors for the strings to be extracted
    let row_selector = Selector::parse("#infoBox > div.items-center").unwrap();
    let span_selector = Selector::parse("span").unwrap();

    // TODO Views, likes and dislikes are only available using another POST request

    // Extract the title
    let title = extract_title(&text).expect("Couldn't extract title");

    // Extract the ratings (upvotes, downvotes, and favorites)
    let ratings = extract_ratings(&text).expect("Couldn't extract ratings");

    // TODO handle chapters (part 1,2,3,...)

    let comment_count = extract_comment_count(&text).expect("Couldn't extract comment count");

    let post_id = extract_post_id(&text).expect("Couldn't extract post id");

    let chapters = extract_chapters(&text);

    // Extract all metadata
    // TODO avoid the use of HTML parsing here if possible
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

    // Fill the data structure for the JSON document to be exported
    let data = ExportV5 {
        hdpc_dl_version: 5,
        program_version: constants::VERSION,
        post_id,
        title: &title,
        upvotes: ratings.upvotes,
        downvotes: ratings.downvotes,
        favorites: ratings.favorites,
        comment_count,
        download_date: Utc::now().to_rfc3339(),
        source_url: url,
        metadata: &metadata,
        chapters,
        picture_urls: &picture_urls,
    };

    // Serialize the data to JSON
    let serialized = serde_json::to_string_pretty(&data).unwrap();

    // Build-a-path
    let path = dest.to_owned() + "/" + &title;

    // Create the destination folder if it doesn't exist
    std::fs::create_dir_all(std::path::Path::new(&path))
        .expect("Failed to create directory.\nTry to specify another path.\n");

    // The JSON path
    let json_path = path.clone() + "/hdpc-info.json";

    // Write the JSON file to disk
    std::fs::write(&json_path, serialized)
        .expect("Failed to create the JSON file.\nTry to specify another path.\n");

    // Log successful JSON file creation
    println!("{padding}Created JSON file at \"{}\"", &json_path);

    // Return if --json-only was specified
    if json_only {
        return Ok(());
    }

    // Only print an empty line if --json-only was not specified
    println!();

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

        match verbosity {
            0 => {
                println!(
                    "{padding}{:03}/{:03} ({:3.0}%)",
                    i + 1,
                    picture_urls.len(),
                    ((i as f32 + 1.) / picture_urls.len() as f32) * 100.
                );
            }
            1 => {
                println!(
                    "{padding}Wrote file {:03}/{:03} ({:3.0}%): {}",
                    i + 1,
                    picture_urls.len(),
                    ((i as f32 + 1.) / picture_urls.len() as f32) * 100.,
                    file_name,
                );
            }
            _ => {
                println!(
                    "{padding}Wrote file {:03}/{:03} ({:3.3}%): {}",
                    i + 1,
                    picture_urls.len(),
                    ((i as f32 + 1.) / picture_urls.len() as f32) * 100.,
                    file_name,
                );
            }
        };
    }

    println!(
        "{pad}Successfully downloaded all {count} images from \"{title}\".",
        count = picture_urls.len(),
        pad = if use_padding { "\n  " } else { "" },
    );

    // This somehow makes this all work
    Ok(())
}

/// Removes the space and the colon from the end of a string slice
fn remove_colon(s: &str) -> &str {
    if s.len() < 2 || s[s.len() - 2..] != *" :" {
        s
    } else {
        &s[..s.len() - 2]
    }
}

/// Crawls an entire search results page and downloads everything
pub async fn crawl_download(
    url: &str,
    dest: &str,
    verbosity: u64,
    json_only: bool,
    limit: usize,
    skip: usize,
    paging: bool,
) -> Result<(), anyhow::Error> {
    // Create a client to make requests with
    let client = reqwest::Client::new();

    let text = client.get(url).send().await?.text().await?;

    let res_pages = {
        let mut res_pages = if paging {
            extract_res_page_links(&text)
        } else {
            vec![]
        };

        if res_pages.is_empty() {
            // If the result only contains one page (without pagination) add it manually
            res_pages.push(ResPage { url, number: 1 })
        }
        res_pages
    };

    // Calculate the number of available posts & build a target list
    let mut targets = vec![];
    for page in res_pages
        .iter()
        .skip(skip / constants::TARGETS_PER_PAGE)
        .take(div_ceil_patch(limit, constants::TARGETS_PER_PAGE))
    {
        let remaining_targets = limit.saturating_sub(targets.len());
        if remaining_targets == 0 && limit != 0 {
            break;
        }

        // TODO implement skip properly

        // Collect all URLs to download
        let text = client.get(page.url).send().await?.text().await?;
        let mut page_contents = extract_target_links(text);
        println!(
            "Collected {post_count: >2} posts from page {current_page: >4}; {total: >4} in total",
            post_count = page_contents.len(),
            current_page = page.number,
            total = targets.len(),
        );
        targets.append(&mut page_contents);
    }

    if targets.is_empty() {
        println!(
            "Skipped all {num_pages} pages because {skip} posts had to be skipped.\nOperation completed.",
            num_pages = res_pages.len()
        );
        return Ok(());
    }

    // Downloads all targets
    let mut total_downloads: usize = 0;
    for target in targets.iter() {
        println!(
            "\nBatch download: {at}/{of} ({percentage:3.0}%)",
            at = total_downloads + 1,
            of = targets.len(),
            percentage = ((total_downloads as f32 + 1.) / targets.len() as f32) * 100.
        );

        // Download the target
        download_from_url(&target.url, dest, verbosity, json_only, true).await?;

        // Increment the download count
        total_downloads += 1;
    }

    println!("\nDownloaded all {total_downloads} targeted posts, (skip={skip}, limit={limit})\nOperation completed.",);

    Ok(())
}

/// Like branchless div_ceil but with branches because the std library marks its function as unstable
fn div_ceil(lhs: usize, rhs: usize) -> usize {
    lhs / rhs + if lhs % rhs == 0 { 0 } else { 1 }
}

fn div_ceil_patch(lhs: usize, rhs: usize) -> usize {
    if lhs != 0 {
        div_ceil(lhs, rhs)
    } else {
        usize::MAX
    }
}
