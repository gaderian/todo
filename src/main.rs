extern crate time;
extern crate regex;
use std::error::Error;
use std::fs::{File,OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader,BufWriter};
use std::env;
use std::process;
use std::fmt;
use regex::Regex;

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
        "list" => list_tasks(args),
        "add" => add_task(args),
        "filter" => filter(args),
        "rm" => remove_tasks(args),
        "done" => complete_tasks(args),
        _ => println!("Not implemented"),
    };
}

fn complete_tasks(args: env::Args) {
    let mut numbered: Vec<ALine> = numbered_lines();
    //let mut iterator = numbered.iter();

    for arg in args {
        numbered = numbered.iter()
            .map(|x| {
                if x.0 == str::parse::<u32>(arg.as_ref()).unwrap() {
                    let mut new_string = String::from("x ");
                    new_string.push_str(x.1.as_ref());
                    ALine(x.0,new_string)
                } else {
                    ALine(x.0,String::from(x.1.as_ref()))
                }
            })
            .collect();
    }

    let file = write_file();
    let _ = file.set_len(0);
    let mut writer = BufWriter::new(file);
    for line in numbered {
        let _ = writer.write(&line.1.as_ref());
        let _ = writer.write(b"\n");
    }
}

/// Removes any specified entries
fn remove_tasks(args: env::Args) {
    let mut numbered: Vec<ALine> = numbered_lines();

    for arg in args {
        numbered.retain(|ref a| a.0 != str::parse::<u32>(arg.as_ref()).unwrap());
    }

    // There has to be an more efficient way of doing this then to rewrite the
    // whole file every time.
    let file = write_file();
    let _ = file.set_len(0);
    let mut writer = BufWriter::new(file);
    for line in numbered {
        let _ = writer.write(&line.1.as_ref());
        let _ = writer.write(b"\n");
    }
}


/// Prints the lines in the todo file sorted and with their linenumber.
fn list_tasks(mut args: env::Args) {
    let mut numbered: Vec<ALine> = numbered_lines();

    let sort: bool = if let Some(a) = args.next() {
        a != "nosort"
    } else {
        true
    };

    if sort {
        numbered.sort_by(|a, b| a.1.cmp(&b.1));
    }

    for line in numbered {
        println!("{}", line);
    }
}

/// Like list_tasks but removes anything not containing the seach words
fn filter(args: env::Args) {
    let mut numbered: Vec<ALine> = numbered_lines();

    for arg in args {
        numbered.retain(|ref a| a.1.contains(&arg[..]));
    }

    numbered.sort_by(|a, b| a.1.cmp(&b.1));
    for line in numbered {
        println!("{}", line);
    }

}

/// Adds a new line to the todo file at the bottom with the specified task
fn add_task(mut args: env::Args) {
    let mut writer = write_file();
    let mut counter = 0;
    let pattern = Regex::new(r"^\([A-Z]\) ").unwrap();

    let date: String = date_string();
    while let Some(text) = args.next() {
        let task: String;
        if pattern.is_match(&text) {
            task = format!("{} {}{}\n", &text[..3], date, &text[3..]);
        } else {
            task = format!("{} {}\n", date, text);
        }
        let _ = writer.write_all(task.as_bytes());
        counter+=1;
    }
    //writer.flush();
    println!("Added {} new entrie(s).", counter);
}

fn date_string() -> String {
    let d = time::now();
    let date = format!("{}-{:02}-{02}", 1900+d.tm_year, 1+d.tm_mon, d.tm_mday);
    date
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

fn numbered_lines() -> Vec<ALine> {
    let file = read_file();
    let lines: Vec<String> = BufReader::new(file).lines()
        .map(|x| x.unwrap())
        .collect();


    let mut tmp: Vec<ALine> = Vec::new();
    let mut i: u32 = 1;
    for line in lines {
        tmp.push(ALine(i,line));
        i=i+1;
    }
    tmp
}
