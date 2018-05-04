#[macro_use]
extern crate structopt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;
use structopt::StructOpt;



fn check_file(f: &std::fs::File) {

    let reader = BufReader::new(f);

    let lines = reader.split(b'\n').map(|l| l.unwrap());

    let mut lineno = 0;
    for line in lines {
        lineno += 1;
        match String::from_utf8(line.clone()) {
            Err(e) => println!("Error in line {}: {}", lineno, e),
            Ok(_) => ()
        }
    }
}

#[derive(StructOpt)]
#[structopt(about="A file checker to validate UTF-8.")]
struct Args {
    #[structopt(short="f", help="File to check for valid UTF-8")]
    infile: String,
}

fn main() {
    let Args { infile } = Args::from_args();

    match File::open(infile) {
        Ok(f) => check_file(&f),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(0x01)
        }
    }
}
