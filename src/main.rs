use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Arguments {
    // path to the file to shred
    file_path: PathBuf,
}

fn main() {
    let args = Arguments::parse();
    println!("path is: {:?}", args.file_path);
}
