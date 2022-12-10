use std::{collections::HashMap, fs, path::Path};

use clap::Parser;
use reqwest::header;
use serde::{Deserialize, Serialize};

// Simple to program to create github Gists
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    //File Path
    #[arg(short, long)]
    file: String,

    //Github Token
    #[arg(short, long)]
    key: String,

    // Gist Description
    #[arg(short, long, default_value_t =String::from(""))]
    description: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if !Path::new(&args.file).exists() {
        println!("No File");
        return;
    }

    let content = match fs::read_to_string(&args.file) {
        Result::Ok(value) => value,
        Result::Err(error) => panic!("{}", error),
    };
    create_gist(
        content,
        Path::new(&args.file).file_name().unwrap().to_str().unwrap(),
        args.key,
    )
    .await;
}

#[derive(Debug, Serialize, Deserialize)]
struct Gist {
    description: String,
    public: bool,
    files: HashMap<String, HashMap<String, String>>,
}

async fn create_gist(content: String, file: &str, key: String) {
    println!("{}", content);
    println!("{}", file);
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
        "X-GitHub-Api-Version",
        header::HeaderValue::from_static("2022-11-28"),
    );

    headers.append(
        "Authorization",
        header::HeaderValue::from_str(&format!("{}{}", "Bearer ", key)).unwrap(),
    );
    let mut file_content: HashMap<String, String> = HashMap::new();
    file_content.insert("content".to_string(), content);

    let mut body: HashMap<String, HashMap<String, String>> = HashMap::new();
    body.insert(file.to_string(), file_content);

    let body = Gist {
        description: "todo!()".to_string(),
        public: true,
        files: body,
    };
    println!("{:?}", headers);
    println!("{:?}", body);

    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.github.com/gists")
        .headers(headers)
        .json(&body)
        .send()
        .await;

    println!("{:?}", resp);
}
