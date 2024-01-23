use std::{thread, time::Duration};

#[cfg(feature = "python_ffi")]
use crate::bypass::http_get_bypassed;
use crate::{
    constants,
    data::*,
    parser::{
        self, extract_chapters, extract_comment_count, extract_from_infobox_row,
        extract_image_urls, extract_info_box_rows, extract_post_id, extract_res_page_links,
        extract_target_links, extract_title, get_api_view,
    },
};
use anyhow::Context;
use chrono::prelude::*;
use tokio::{fs, io::AsyncWriteExt};
use tracing::info;

/// Downloads comic(s) from given URL(s) to a target directory
pub async fn download_from_urls(
    urls: Vec<String>,
    dest: String,
    verbosity: u64,
    json_only: bool,
    use_padding: bool,
    #[cfg(feature = "python_ffi")] use_python_bypass: bool,
    get_comments: bool,
) -> Result<(), anyhow::Error> {
    info!("Downloading pre-defined list of URLs");

    let max = urls.len();
    for (n, url) in urls.iter().enumerate().map(|(n, url)| (n + 1, url)) {
        println!("Download {n:02}/{max:02}");
        download_from_url(
            url.to_string(),
            dest.clone(),
            verbosity,
            json_only,
            use_padding,
            #[cfg(feature = "python_ffi")]
            use_python_bypass,
            get_comments,
        )
        .await?;
    }
    println!("Download done.");

    Ok(())
}

pub async fn download_from_url(
    url: String,
    dest: String,
    verbosity: u64,
    json_only: bool,
    use_padding: bool,
    #[cfg(feature = "python_ffi")] use_python_bypass: bool,
    get_comments: bool,
) -> Result<(), anyhow::Error> {
    info!("Getting target {url}");

    let padding = if use_padding { "  " } else { "" };

    // Inform the user about the actions to be taken
    println!("{padding}Destination: {dest}");
    println!("{padding}URL: {url}");

    #[cfg(feature = "python_ffi")]
    if use_python_bypass {
        pyo3::prepare_freethreaded_python();
        info!("Prepared the Python FFI");
    }

    // Create a client to make requests with
    let client = reqwest::Client::new();

    // Request the HTML file from the server
    #[cfg(feature = "python_ffi")]
    let text = if use_python_bypass {
        http_get_bypassed(url.clone())?
    } else {
        info!("Downloading HTML with Reqwest from {url}",);
        client
            .get(url.clone())
            .send()
            .await?
            .text()
            .await?
            .to_string()
    };

    info!("Downloading HTML with Reqwest from {url}",);
    #[cfg(not(feature = "python_ffi"))]
    let text = client
        .get(url.clone())
        .send()
        .await?
        .text()
        .await?
        .to_string();

    // The URLs of the pictures to be downloaded
    let picture_urls = extract_image_urls(&text);

    if verbosity >= 6 {
        println!("{text}");
    }

    // Extract the title
    let title = extract_title(&text).context("Couldn't extract title")?;

    // Get the stats from the API (upvotes, downvotes, views, and favorites)
    let api_stats = get_api_view(&client, &url).await?;

    let comment_count = extract_comment_count(&text).context("Couldn't extract comment count")?;

    let post_id = extract_post_id(&text).context("Couldn't extract post id")?;

    let chapters = extract_chapters(&text);

    // Extract all metadata
    let info_rows = extract_info_box_rows(&text)
        .into_iter()
        .map(extract_from_infobox_row)
        .collect();

    let comments = if get_comments {
        let comments = parser::get_comments(post_id, &client).await?;
        println!("{padding}Got {} comments", comments.len());
        Some(comments)
    } else {
        info!("Skipped comment download");
        None
    };

    // Fill the data structure for the JSON document to be exported
    let data = ExportV7 {
        hdpc_dl_version: 7,
        program_version: constants::VERSION,
        post_id,
        title: &title,
        api_stats,
        comment_count,
        download_date: Utc::now().to_rfc3339(),
        source_url: &url,
        metadata: &info_rows,
        chapters,
        picture_urls: &picture_urls,
        comments,
    };

    // Serialize the data to JSON
    let serialized = serde_json::to_string_pretty(&data).unwrap();
    info!("Serialized the export");

    // Build-a-path
    let path = dest.to_owned() + "/" + &title;

    // Create the destination folder if it doesn't exist
    std::fs::create_dir_all(std::path::Path::new(&path))
        .context("Failed to create directory.\nTry to specify another path.\n")?;

    // The JSON path
    let json_path = path.clone() + "/hdpc-info.json";

    // Write the JSON file to disk
    std::fs::write(&json_path, serialized)
        .context("Failed to create the JSON file.\nTry to specify another path.\n")?;

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

/// Crawls an entire search results page and downloads everything
pub async fn crawl_download(
    url: &str,
    dest: &str,
    verbosity: u64,
    json_only: bool,
    limit: usize,
    skip: usize,
    paging: bool,
    max_retries: usize,
    no_download: bool,
    #[cfg(feature = "python_ffi")] use_python_bypass: bool,
    get_comments: bool,
) -> Result<(), anyhow::Error> {
    // Create a client to make requests with
    #[cfg(not(feature = "python_ffi"))]
    let client = reqwest::Client::new();

    #[cfg(feature = "python_ffi")]
    let text = http_get_bypassed(url)?;
    #[cfg(not(feature = "python_ffi"))]
    let text = client
        .get(url.clone())
        .send()
        .await?
        .text()
        .await?
        .to_string();

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

    println!("Query returned {} result pages", res_pages.len());

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
        #[cfg(feature = "python_ffi")]
        let text = http_get_bypassed(page.url)?;
        #[cfg(not(feature = "python_ffi"))]
        let text = client
            .get(page.url.clone())
            .send()
            .await?
            .text()
            .await?
            .to_string();
        let mut page_contents = extract_target_links(text);
        println!(
            "Collected {post_count: >2} posts from page {current_page: >4}; {total: >4} in total",
            post_count = page_contents.len(),
            current_page = page.number,
            total = targets.len(),
        );
        targets.append(&mut page_contents);
        // thread::sleep(Duration::from_secs(3));
    }

    let export = CrawlResultV5 {
        hdpc_dl_version: 6,
        program_version: constants::VERSION,
        source_url: url,
        download_date: Utc::now().to_rfc3339(),
        posts: &targets,
    };

    // Serialize the data to JSON
    let serialized = serde_json::to_string_pretty(&export).unwrap();

    // Build-a-path
    let path = dest.to_owned();

    // Create the destination folder if it doesn't exist
    std::fs::create_dir_all(std::path::Path::new(&path))
        .context("Failed to create directory.\nTry to specify another path.\n")?;

    // TODO allow users to specify the name
    let crawl_export_name = Utc::now().to_rfc3339().replace(" ", "_");

    // The JSON path
    let json_path = path.clone() + "/" + &crawl_export_name + "_crawl_results.json";

    // Write the JSON file to disk
    std::fs::write(&json_path, serialized)
        .context("Failed to create the JSON file.\nTry to specify another path.\n")?;

    // Log successful JSON file creation
    println!("Created JSON file with crawl results at \"{}\"", &json_path);
    println!("from path {url}");

    if no_download {
        return Ok(());
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

        let mut retries = 0;

        // Download the target
        while let Err(e) = download_from_url(
            target.url.clone(),
            dest.to_owned(),
            verbosity,
            json_only,
            true,
            #[cfg(feature = "python_ffi")]
            use_python_bypass,
            get_comments,
        )
        .await
        {
            retries += 1;

            if retries > max_retries {
                return Err(e);
            }

            println!(
                "Couldn't extract title; waiting 10 seconds before retry ({retries}/{max_retries})"
            );
            thread::sleep(Duration::from_secs(10));
        }

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
