use std::error::Error;
use std::fs::{File,OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader,BufWriter};
use std::env;
use std::process;
use std::fmt;

struct a_line(u32,String);

impl fmt::Display for a_line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

fn main () {
    // Create a path to the desired file

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
        "list" => list_entries(),
        "add" => add_entry(args),
        _ => println!("Not implemented"),
    };


}

fn list_entries() {
    let file = read_file();
    let mut lines: Vec<String> = BufReader::new(file).lines()
        .map(|x| x.unwrap())
        .collect();

    let mut with_nr = {
        let mut tmp: Vec<a_line> = Vec::new();
        let mut i: u32 = 1;
        for line in lines {
            tmp.push(a_line(i,line));
            i=i+1;
        }
        tmp
    };

    with_nr.sort_by(|a, b| a.1.cmp(&b.1));
    for line in with_nr {
        println!("{}", line);
    }
}

fn add_entry(mut args: env::Args/*std::iter::Iterator<Item = String>*/) {
    println!("not realy adding...");
    let mut new_entry = String::new();
    let mut writer = write_file();
    while let Some(mut word) = args.next() {
        println!("{}", word);
        word.push('\n');
        writer.write_all(word.as_bytes());
    }
    writer.flush();
    println!("{}", new_entry);

}

fn write_file() -> File {
    let mut option = OpenOptions::new();
    option.write(true);
    option.append(true);
    option.create(true);
    get_file(option)
}

fn read_file() -> File {
    let mut option = OpenOptions::new();
    option.read(true);
    get_file(option)
}

fn get_file(option: OpenOptions) -> File {
    let home = match env::home_dir() {
        Some(path) => path,
        None => panic!("Cant get your home dir."),
    };

    let mut tmp1 = home.to_path_buf();
    tmp1.push("todo.txt");
    let path = tmp1.as_path();
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match option.open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    file
}
