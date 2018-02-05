extern crate regex;
extern crate caseless;
#[macro_use]
extern crate lazy_static;

mod types;
mod sln_parser;

use std::env;
use std::fs::File;
use caseless::default_caseless_match_str;

use types::FileType;
use sln_parser::parse_sln;

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        usage();
        ::std::process::exit(1);
    }

    let root_file = args.nth(1).unwrap();
    match get_file_type(&root_file).unwrap() {
        FileType::Sln => {
            let file = File::open(root_file).unwrap();
            let projects = parse_sln(file);
            for p in projects.iter().filter(|p| p.path.ends_with(".csproj")) {
                println!("{} {}", p.name, p.path);
            }
        },
        _ => println!("Unknown file type")
    };
}

fn usage() {
    println!("Usage: vsan <sln | csproj file>");
}

fn get_file_type(file_path: &str) -> Option<FileType> {
    use std::path::Path;

    Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            if default_caseless_match_str(ext, "sln") {
                FileType::Sln
            } else if default_caseless_match_str(ext, "csproj") {
                FileType::Csproj
            } else {
                FileType::Unknown
            }
        })
}