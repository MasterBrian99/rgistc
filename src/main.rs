use std::{fs, path::Path};

use clap::{builder::Str, Parser};

// Simple to program to create github Gists
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    //File Path
    #[arg(short, long)]
    file: String,

    //Github Token
    #[arg(short, long,default_value_t =String::from(""))]
    key: String,

    // Gist Description
    #[arg(short, long, default_value_t =String::from(""))]
    description: String,
}

fn main() {
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
    )
}

fn create_gist(content: String, file: &str) {
    println!("{}", content);
    println!("{}", file)
}
