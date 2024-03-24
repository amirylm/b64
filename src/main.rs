#[path = "./encoding.rs"]
mod encoding;
use std::fs;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("b64")
        .version("0.0.1")
        // .author("Amir Y <amirylm.dev@gmail.com>")
        .about("Base 64 en/decoding")
        .arg(Arg::new("cmd")
                .short('c')
                .long("cmd")
                .help("Whether to encode (e) or decode (d)"))
        .arg(Arg::new("file")
                .short('f')
                .long("file")
                .help("File containing data to encode or decode"))
        .arg(Arg::new("input")
                .short('i')
                .long("input")
                .help("Five less than your favorite number"))
        .get_matches();

    let d = String::from("");
    let file_path = matches.get_one::<String>("file").unwrap_or(&d);
    let file_contents;
    let data: &str = if file_path.is_empty() {
        matches.get_one::<String>("input").unwrap()
    } else { 
        file_contents = fs::read_to_string(file_path).unwrap_or(String::from(""));
        file_contents.as_str()
    };

    let cmd = matches.get_one::<String>("cmd").unwrap();

    match cmd.as_str() {
        "e" => encoding::encode(data),
        "encode" => encoding::encode(data),
        "d" => encoding::decode(data),
        "decode" => encoding::decode(data),
        _ => println!("Unknown command")
    }
    
}