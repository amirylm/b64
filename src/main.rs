#[path = "./encoding.rs"]
mod encoding;
use std::fs;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("b64")
        .version("0.0.1")
        // .author("Amir Y <amirylm.dev@gmail.com>")
        .about("Base 64 en/decoding")
        .subcommand(Command::new("encode")
                .about("Encode data")
                .alias("e")
                .arg(Arg::new("file")
                        .short('f')
                        .long("file")
                        .help("File containing data to encode"))
                .arg(Arg::new("input")
                        .short('i')
                        .long("input")
                        .help("Raw string input")))
        .subcommand(Command::new("decode")
                .about("Decode data")
                .alias("d")
                .arg(Arg::new("file")
                        .short('f')
                        .long("file")
                        .help("File containing data to decode"))
                .arg(Arg::new("input")
                        .short('i')
                        .long("input")
                        .help("Raw string input")))
            .get_matches();

        matches.subcommand().map(|(cmd, matches)| {
            let empty_str = String::from("");
            let file_path = matches.get_one::<String>("file").unwrap_or(&empty_str);
            let file_contents;
            let data: &str = if file_path.is_empty() {
                matches.get_one::<String>("input").unwrap_or(&empty_str)
            } else { 
                file_contents = fs::read_to_string(file_path).unwrap_or(empty_str);
                file_contents.as_str()
            };
            match cmd {
                "encode" => encoding::encode(data),
                "decode" => encoding::decode(data),
                _ => println!("Unknown command")
            }
        });
}