use std::io;
use std::fs;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::env;
use ferris_says::say;
use eval::eval;

pub fn help() {
    println!("This is the help screen.");
}

pub fn list() -> std::io::Result<()> {
    let read_dir = fs::read_dir(env::current_dir().unwrap())?;
    for entry in read_dir {
        let entry_dir = entry?;
        println!("{}", entry_dir.file_name().into_string().unwrap());
    }

    Ok(())
}

pub fn change_directory(params: Vec<&str>) {
    if params.get(0).is_none() {
        return
    }

    if let Err(e) = env::set_current_dir(params[0]) {
        eprintln!("{}", e);
    }
}

pub fn touch(params: Vec<&str>) {
    if params.get(0).is_none() {
        return
    }

    println!("Created file with filename {}", params[0]);
}

pub fn ferris(params: Vec<&str>) {
    if params.get(0).is_none() {
        return
    }

    let stdout = io::stdout();
    let text = params[0];
    let width = text.len();
    let writer = BufWriter::new(stdout.lock());

    say(params[0], width, writer).unwrap();
}

pub fn calc(params: Vec<&str>) {
    if params.get(0).is_none() {
        return
    }

    let re = regex::Regex::new(r"[0-9]+|[+\+\-\/\*]").unwrap();
    let mut sequence = String::new();

    for param in params {
        if let Some(caps) = re.captures(param) {
            sequence.push_str(param);
        } else {
            println!("Invalid sequence {}", param);
            return
        }
    }

    match eval(sequence.as_str()) {
        Ok(result) => {
            println!("{}", result);
        },
        Err(error) => {
            println!("{}", error);
        }
    }

}