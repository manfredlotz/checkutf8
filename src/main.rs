#[macro_use]
extern crate structopt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;
use structopt::StructOpt;



fn check_file(filename: &str) {

    let file = File::open(filename);
    match file {
        Ok(f) => 
        {
            let reader = BufReader::new(&f);

            let lines = reader.split(b'\n').map(|l| l.unwrap());

            let mut line_no = 0;
            for line in lines {
                line_no = line_no + 1;
                match String::from_utf8(line.clone()) {
                    Err(e) => println!("Errer in line {}: {}", line_no, e),
                    Ok(_) => ()
                }
            }}
        Err(e) => {
            println!("Error: {}", e);
            process::exit(0x01)
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


    //let mut line_no = 0;

    // for line in slurp::iterate_all_lines(&infile) {
    //     line_no = line_no + 1;
    //     println!("{:?}",line);
    //     match line {
    //         Err(e) => match e.kind() {
    //             std::io::ErrorKind::InvalidData => {  println!("Line {}: {}",line_no, e);
    //                                                   // Will print "No inner error".
    //                                                   print_error(&Error::last_os_error());
    //                                                   // Will print "Inner error: ...".
    //                                                   print_error(&Error::new(ErrorKind::Other, "oh no!"));
    //             },
    //             std::io::ErrorKind::PermissionDenied => {
    //                 println!("Error: {}", e);
    //                 process::exit(0x01)
    //             },
    //             _ => {
    //                 println!("Error: {}",e);
    //                 process::exit(0x02)
    //             }
    //         },
    //         Ok(_line) => ()
    //     }
    // }
    check_file(&infile);
}
