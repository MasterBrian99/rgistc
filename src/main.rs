mod log;

use std::{fs, path::Path};

use clap::Parser;
use log::response_logs;

// Simple to program to create github Gists
#[derive(Parser, Debug)]
#[command(name = "Rgistc")]
#[command(author = "Pasindu P Konghawaththa <pasindudixon25@gmail.com>")]
#[command(about = "Simple to program to create github Gists", long_about = None)]
#[command(version)]
struct Args {
    //Name and content for the file that make up the gist
    #[arg(short, long)]
    file: String,

    //Github Token
    #[arg(short, long)]
    key: String,

    // Description of the gist
    #[arg(short, long, default_value_t =String::from(""))]
    description: String,
}

fn main() {
    let args = Args::parse();

    if !Path::new(&args.file).exists() {
        response_logs::log_common_errors("Unable to Find this File...");
        return;
    }

    let content = match fs::read_to_string(&args.file) {
        Result::Ok(value) => value,
        Result::Err(error) => {
            response_logs::log_common_errors(&error.to_string());
            return;
        }
    };
    create_gist(
        content,
        Path::new(&args.file).file_name().unwrap().to_str().unwrap(),
        args.key,
        args.description,
    );
}

fn create_gist(content: String, file_name: &str, key: String, description: String) {
    let resp = ureq::post("https://api.github.com/gists")
        .set("Accept", " application/vnd.github+json")
        .set("Authorization", &format!("{}{}", "Bearer ", key))
        .set("X-GitHub-Api-Version", "2022-11-28")
        .send_json(ureq::json!({
            "description":description,
            "public":false,
            "files":{
                file_name:{"content":content}
            }
        }));
    match resp {
        Ok(_) => {
            response_logs::success_response();
        }
        Err(ureq::Error::Status(code, response)) => response_logs::log_api_errors(code, response),
        Err(err) => response_logs::log_common_errors(&err.to_string()),
    }
}
