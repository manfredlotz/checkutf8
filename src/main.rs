#[macro_use]
extern crate structopt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use structopt::StructOpt;

use std::path::Path;


fn check_file(fname: &std::path::Display, f: &std::fs::File) {

    let reader = BufReader::new(f);

    let lines = reader.split(b'\n').map(|l| l.unwrap());

    let mut lineno = 0;
    for line in lines {
        lineno += 1;
        match String::from_utf8(line.clone()) {
            Err(e) => println!("{} : Error in line {}: {}", fname, lineno, e),
            Ok(_) => ()
        }
    }
}

#[derive(StructOpt)]
#[structopt(about="A file checker to validate UTF-8.")]
struct Args {
    #[structopt(help="Files to check for valid UTF-8", raw(required = "true"))]
    files: Vec<String>,
}

fn main() {
    let Args { files } = Args::from_args();

    for file in files {
        let path = Path::new(&file);
        let display = path.display();

        match File::open(&path) {
            Ok(f) => check_file(&display, &f),
            Err(e) => {
                println!("{} : Error: {}", display, e);
            }
        }
    }
}
