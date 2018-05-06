#[macro_use]
extern crate structopt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;
use structopt::StructOpt;

use std::path::Path;


fn check_file(fname: &str, f: &std::fs::File) -> bool {
    let reader = BufReader::new(f);

    let lines = reader.split(b'\n').map(|l| l.unwrap());
    let mut result = true;

    let mut lineno = 0;
    for line in lines {
        lineno += 1;
        match String::from_utf8(line.clone()) {
            Err(e) => { println!("{} : Error in line {}: {}", fname, lineno, e);
                        result = false
            },
            Ok(_) => ()
        }
    }

    return result;
}

#[derive(StructOpt)]
#[structopt(about="A file checker to validate UTF-8.")]
struct Args {
    #[structopt(help="Files to check for valid UTF-8", raw(required = "true"))]
    files: Vec<String>,
}

fn main() {
    let Args { files } = Args::from_args();

    let mut highrc = 0x00;

    for file in files {
        let path = Path::new(&file);
        if !path.is_file() {
            println!("{} : not a file", file);
            highrc = 0x01;
            continue;
        }

        match File::open(&path) {
            Ok(f) => if !check_file(&file, &f) {
                highrc = 0x01;
            },
            Err(e) => {
                println!("{} : Error: {}", file, e);
                highrc = 0x01;
            }
        }
    }

    process::exit(highrc)
}
