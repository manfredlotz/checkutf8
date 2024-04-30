use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process;

use clap::{Parser, ValueHint};

fn check_file(fname: &str, f: &std::fs::File) -> bool {
    let reader = BufReader::new(f);

    let lines = reader.split(b'\n').map(|l| l.unwrap());
    let mut result = true;

    let mut header_printed = false;
    let mut lineno = 0;
    for line in lines {
        lineno += 1;
        if let Err(e) = String::from_utf8(line.clone()) {
            if !header_printed {
                println!("{}: not valid UTF-8", fname);
                header_printed = true;
            }
            println!("   >> line {} {}", lineno + 1, e);
            result = false
        }
    }

    if !header_printed {
        println!("{}: ok", fname);
    }
    result
}

#[derive(Parser, Debug, PartialEq)]
#[clap(author, version, about, long_about = None,arg_required_else_help(true))]
struct Args {
    #[arg(help="Files to check for valid UTF-8", value_hint= ValueHint::FilePath)]
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let mut highrc = 0x00;

    for file in &args.files {
        let path = Path::new(&file);
        if !path.is_file() {
            println!("{} : not a file", file);
            highrc = 0x01;
            continue;
        }

        match File::open(path) {
            Ok(f) => {
                if !check_file(file, &f) {
                    highrc = 0x01;
                }
            }
            Err(e) => {
                println!("{} : Error: {}", file, e);
                highrc = 0x01;
            }
        }
    }

    process::exit(highrc)
}
