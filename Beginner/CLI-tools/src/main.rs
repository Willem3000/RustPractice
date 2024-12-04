use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;
use std::env;
use regex::Regex;

mod commands;

const C_EMPTY: &str = "";
const C_HELP: &str = "help";
const C_LS: &str = "ls";
const C_CD: &str = "cd";
const C_TOUCH: &str = "touch";
const C_FERRIS: &str = "ferris";
const C_CALC: &str = "calc";

fn main() {
    while true {
        print!("{}> ", env::current_dir().unwrap().display());
        io::stdout().flush().unwrap();
        let input = get_input();
        receive_command(input.trim());
    }
    //let _ = write_foo(&input);
    //let _ = read_foo();
}

fn receive_command(raw_command: &str) {
    let re = Regex::new(r#""([^"]*)"|(\S+)"#).unwrap();

    let mut params: Vec<&str> = Vec::new();

    for cap in re.captures_iter(raw_command) {
        if let Some(quoted) = cap.get(1) {
            params.push(quoted.as_str());
        } else if let Some(word) = cap.get(2) {
            params.push(word.as_str());
        }
    }

    let command = params.remove(0);

    match command {
        C_EMPTY => return,
        C_HELP => commands::help(),
        C_LS => commands::list().expect(""),
        C_CD => commands::change_directory(params),
        C_TOUCH => commands::touch(params),
        C_FERRIS => commands::ferris(params),
        C_CALC => commands::calc(params),
        _ => println!("Command '{}' not recognized.", command)
    }
}

fn get_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    return input;
}

fn write_foo(text: &String) -> io::Result<()> {
    let f = File::create("foo.txt")?;
    let mut writer = BufWriter::new(f);
    writer.write(text.as_bytes())?;

    Ok(())
}

fn read_foo() -> io::Result<()> {
    let f = File::open("foo.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;

    println!("{buffer}");
    Ok(())
}