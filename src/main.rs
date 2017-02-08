use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;
use std::io;
use std::env;
use std::path::{Path, PathBuf};
use std::str::SplitWhitespace;


fn main () {
    // Create a path to the desired file

    let mut args = env::args();
    match args.next() {
        None => println!("I don't think its possible to get here"),
        _ => (),
    }

    match args.next() {
        Some("list") => list_entries(),
        //Some("add") => add_entry(args),
        _ => println!("Not implemented"),
    };




    /*
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("NOPE");
    let mut input = input.split_whitespace();
    */

}

fn list_entries() {
    let mut file = get_file();
    let lines = BufReader::new(file).lines();
    for line in lines {
        println!("{}", line.unwrap());
    }
}

fn add_entry(entry: std::iter::Iterator<String>) {
    println!("not realy adding...");

}

fn get_file() -> File {
    let home = match env::home_dir() {
        Some(path) => path,
        None => panic!("Cant get your home dir."),
    };

    let mut tmp1 = home.to_path_buf();
    tmp1.push("todo.txt");
    let path = tmp1.as_path();
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    file
}
