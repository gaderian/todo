use std::error::Error;
use std::fs::{File,OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::process;
use std::fmt;

struct ALine(u32,String);

impl fmt::Display for ALine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

fn main () {
    let mut args = env::args();
    match args.next() {
        None => println!("I don't think its possible to get here"),
        _ => (),
    }
    
    let arg = match args.next() {
        Some(a) => a,
        None =>  process::exit(1),
    };

    match arg.as_ref() {
        "list" => list_entries(args),
        "add" => add_entry(args),
        "filter" => filter(args),
        _ => println!("Not implemented"),
    };
}

/// Prints the lines in the todo file sorted and with their linenumber.
fn list_entries(mut args: env::Args) {
    let file = read_file();
    let lines: Vec<String> = BufReader::new(file).lines()
        .map(|x| x.unwrap())
        .collect();

    let mut with_nr = {
        let mut tmp: Vec<ALine> = Vec::new();
        let mut i: u32 = 1;
        for line in lines {
            tmp.push(ALine(i,line));
            i=i+1;
        }
        tmp
    };

    let sort: bool = if let Some(a) = args.next() {
        a != "nosort"
    } else {
        true
    };

    if sort {
        with_nr.sort_by(|a, b| a.1.cmp(&b.1));
    }

    for line in with_nr {
        println!("{}", line);
    }
}

fn filter(mut args: env::Args) {
    let file = read_file();
    let mut lines: Vec<String> = BufReader::new(file).lines()
        .map(|x| x.unwrap())
        .collect();

    let mut with_nr = {
        let mut tmp: Vec<ALine> = Vec::new();
        let mut i: u32 = 1;
        for line in lines {
            tmp.push(ALine(i,line));
            i=i+1;
        }
        tmp
    };

    for arg in args {
        with_nr.retain(|ref a| a.1.contains(&arg[..]));
    }

    with_nr.sort_by(|a, b| a.1.cmp(&b.1));
    for line in with_nr {
        println!("{}", line);
    }

}

/// Adds a new line to the todo file at the bottom with the specified entry
fn add_entry(mut args: env::Args) {
    let mut writer = write_file();
    let mut counter = 0;

    while let Some(mut word) = args.next() {
        word.push('\n');
        writer.write_all(word.as_bytes());
        counter+=1;
    }
    //writer.flush();
    println!("Added {} new entrie(s).", counter);
}

/// Returns a file opened for appending
fn write_file() -> File {
    let mut option = OpenOptions::new();
    option.write(true);
    option.append(true);
    option.create(true);
    get_file(option)
}

/// Returns a file opened for reading
fn read_file() -> File {
    let mut option = OpenOptions::new();
    option.read(true);
    get_file(option)
}

/// Opens a file with options set in the passed std::fs::OpenOptions
fn get_file(option: OpenOptions) -> File {
    let home = match env::home_dir() {
        Some(path) => path,
        None => panic!("Cant get your home dir."),
    };

    let mut tmp1 = home.to_path_buf();
    tmp1.push("todo.txt");
    let path = tmp1.as_path();
    let display = path.display();

    let file = match option.open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    file
}
