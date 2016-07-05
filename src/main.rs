
extern crate rustc_serialize;
extern crate docopt;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::process;

use docopt::Docopt;

const USAGE: &'static str = "
Doublon
Dedup lines in file.

Usage:
  doublon <file>
  doublon (-h | --help)

Options:   
  -h --help     Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    let mut slices = file_as_vector(&args.arg_file);
    slices.dedup();
    file_put_contents(&args.arg_file, &slices.join("\n"));
}

// Get file contents as a Vector of Strings
fn file_as_vector(filename: &str) -> Vec<String> {
    let mut s = String::new();
    match open_or_die(filename).read_to_string(&mut s) {
        Err(why) => die(&format!("Couldn't read file {}: {}", filename, why.description())),
        Ok(_) => s,
    }.split("\n").map(String::from).collect()
}

// Put a string into a file
fn file_put_contents(filename: &str, contents: &str) {
    match create_or_die(filename).write_all(contents.as_bytes()) {
        Err(why) => die(&format!("Couldn't write file {}: {}", filename, why.description())),
        Ok(_) => println!("Successfully wrote to {}", filename),
    }
}

// Open a file or die
fn open_or_die(filename: &str) -> File {
    File::open(&Path::new(filename))
        .unwrap_or_else(|why| die(&format!("Couldn't open file {}: {}", filename, why.description())))
}

// Create a file or die
fn create_or_die(filename: &str) -> File {
    File::create(&Path::new(filename))
        .unwrap_or_else(|why| die(&format!("Couldn't create file {}: {}", filename, why.description())))
}

// Die with message
fn die(message: &str) -> ! {
    println!("{}", message);
    process::exit(1);
}