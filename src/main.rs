use std::{
    env, io,
    path::PathBuf,
    process::{Command, Stdio},
};

use reqwest::{header::CONTENT_DISPOSITION, Error, Url};
use serde::{Deserialize, Serialize};
use trauma::{download::Download, downloader::DownloaderBuilder};
use validator::Validate;

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("A cross-platform silent installer written in Rust. Enjoy!");
    println!("Enter the URL: ");
    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap_or(0);
    let url = url.trim();

    let app_list = get_apps_list(&url).await.unwrap_or(vec![]);
    let mut downloads: Vec<Download> = vec![];

    for (_pos, item) in app_list.iter().enumerate() {
        let mut file_name = get_filename(item.url.as_str()).await;
        let url = Url::parse(item.url.as_str()).unwrap();

        if file_name.is_empty() {
            file_name = get_filename_from_url(&item.url).unwrap();
        }
        downloads.push(Download::new(&url, &file_name));
    }

    download_files(&downloads).await;
    install_downloaded(&downloads, &app_list);

    Ok(())
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, Validate)]
struct SiinList {
    title: String,
    url: String,
    #[serde(default)]
    alt: String,
}

async fn get_apps_list(url_string: &str) -> Result<Vec<SiinList>, Error> {
    let response = reqwest::get(url_string).await?.text().await?;
    let dummy = vec![];
    let list: Vec<SiinList> = serde_json::from_str(response.as_str()).unwrap_or(dummy);
    Ok(list)
}

async fn get_filename(url: &str) -> String {
    let res = reqwest::get(url).await.unwrap();
    if let Some(cd) = res.headers().get(CONTENT_DISPOSITION) {
        let hv = cd.to_str().unwrap();
        let index = hv.find("filename=").unwrap();
        let file_name = &hv[index + 9..];
        let trimmed_file_name = file_name.trim_matches('"');
        trimmed_file_name.to_string()
    } else {
        "".to_string()
    }
}

async fn download_files(downloads: &Vec<Download>) {
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from("output"))
        .build();

    downloader.download(&downloads).await;
}

fn run_install_commands(command_str: &str) {
    let windows_os = "windows";
    let command_types = {
        if windows_os == std::env::consts::OS {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        }
    };

    let mut command = Command::new(command_types.0);
    command.arg(command_types.1);
    command.arg(command_str);
    command.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    let path = env::current_dir().unwrap();
    let current_dir = path.to_str().unwrap();
    let output_dir = format!("{}\\output", current_dir);

    command.current_dir(output_dir);
    // Way1
    // let mut child = command.spawn().unwrap();
    // child.wait().unwrap();
    // Way2
    let mut child = command
        .spawn()
        .map_err(|e| format!("Error executing the installer: {:?}", e))
        .unwrap();
    let status = child
        .wait()
        .map_err(|e| format!("Error waiting for the installer: {:?}", e))
        .unwrap();
    if status.success() {
        println!("done: {}", command_str);
    } else {
        println!("Err: {} : {}", command_str, status.code().unwrap());
    }
}

fn install_downloaded(downloads: &Vec<Download>, app_list: &Vec<SiinList>) {
    for (pos, item) in downloads.iter().enumerate() {
        let install_arg = &app_list[pos].alt;
        let mut command_str = String::default();
        if install_arg.is_empty() {
            if item.filename.ends_with(".exe") {
                command_str = format!("{} /S", item.filename);
            } else if item.filename.ends_with(".msi") {
                command_str = format!("msiexec /i {} /qn", item.filename,);
            }
        } else {
            command_str = format!("{} {}", item.filename, install_arg);
        }

        println!("Installing {}", item.filename);
        run_install_commands(&command_str);
    }
}

fn get_filename_from_url(url_str: &str) -> Option<String> {
    if let Some(start_pos) = url_str.rfind('/') {
        if let Some(end_pos) = url_str.rfind('?') {
            if end_pos > start_pos {
                return Some(url_str[start_pos + 1..end_pos].to_string());
            }
        } else {
            return Some(url_str[start_pos + 1..].to_string());
        }
    }
    None
}
