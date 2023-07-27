use std::io;

use reqwest::{header::CONTENT_DISPOSITION, Error, Url};
use serde::{Deserialize, Serialize};
use trauma::download::Download;
use validator::Validate;

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("A cross-platform silent installer written in Rust. Enjoy!");
    println!("Enter the URL: ");
    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap_or(0);
    let url = url.trim();
    let list = get_apps_list(&url).await.unwrap_or(vec![]);
    let mut downloads: Vec<Download> = vec![];

    for (pos, item) in list.iter().enumerate() {
        let file_name = get_filename(item.url.as_str()).await;
        downloads.push(Download::new(
            &Url::parse(item.url.as_str()).unwrap(),
            &file_name,
        ));
    }

    download_files(downloads).await;

    Ok(())
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, Validate)]
struct SiinList {
    title: String,
    url: String,
}

async fn get_apps_list(url_string: &str) -> Result<Vec<SiinList>, Error> {
    let response = reqwest::get(url_string).await?.text().await?;
    let dummy = vec![];
    let list: Vec<SiinList> = serde_json::from_str(response.as_str()).unwrap_or(dummy);
    Ok(list)
}

async fn get_filename(url: &str) -> String {
    let res = reqwest::get(url).await.unwrap();
    let cd = res.headers().get(CONTENT_DISPOSITION).unwrap();
    let hv = cd.to_str().unwrap();
    let index = hv.find("filename=").unwrap();
    let file_name = &hv[index + 9..];
    let trimmed_file_name = file_name.trim_matches('"');
    trimmed_file_name.to_string()
}

async fn download_files(downloads: Vec<Download>) {
    //
}
