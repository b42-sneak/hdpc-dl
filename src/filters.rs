use crate::constants;

// TODO remove the following allow
#[allow(unreachable_code)]
pub async fn get_filters() -> Result<(), anyhow::Error> {
    todo!("The download of filters from the search engine");

    let client = reqwest::Client::new();

    // let (artist, category, characters, groups, tags, parody) = get_filters_text(client).await?;
    let mut res = get_filters_text(client).await?;

    // Convert the whatever's into valid JSON documents
    for text in res.iter_mut() {
        text.pop();
        text.remove(0);
        *text = text.replace("\\", "");
    }

    Ok(())
}

async fn get_filters_text(client: reqwest::Client) -> Result<[String; 6], anyhow::Error> {
    let artist = client
        .post(constants::API_FILTER_PATH)
        .body("tax=artist")
        .send()
        .await?
        .text()
        .await?;
    let category = client
        .post(constants::API_FILTER_PATH)
        .body("tax=category")
        .send()
        .await?
        .text()
        .await?;
    let characters = client
        .post(constants::API_FILTER_PATH)
        .body("tax=characters")
        .send()
        .await?
        .text()
        .await?;
    let groups = client
        .post(constants::API_FILTER_PATH)
        .body("tax=groups")
        .send()
        .await?
        .text()
        .await?;
    let tags = client
        .post(constants::API_FILTER_PATH)
        .body("tax=tags")
        .send()
        .await?
        .text()
        .await?;
    let parody = client
        .post(constants::API_FILTER_PATH)
        .body("tax=parody")
        .send()
        .await?
        .text()
        .await?;
    Ok([artist, category, characters, groups, tags, parody])
}
