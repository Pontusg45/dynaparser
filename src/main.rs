use std::fs::File;
use std::io::Read;
use regex::Regex;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Instructions {
    groups: Groups,
    session: Session
}

#[derive(Serialize, Deserialize)]
struct Session {
    start : String,
    finished : String,
    error : String
}

// TODO make fields dynamic based on the schema
#[derive(Serialize, Deserialize)]
struct Groups {
    date_format : String,
    name : String,
    id : String,
    output : String
}

fn main() {
    let schema_str = get_string_from_file("./res/schema.json");

    let log_str = get_string_from_file("./res/log.log");

    let mut counter = 0;

    let deserialized: Instructions = serde_json::from_str(&schema_str).unwrap();

    let groups = deserialized.groups;

    // TODO make this into a function and in a loop per group in the schema to make it more dynamic
    let mut regex_string = String::from(r"(?P<date>");
    regex_string.push_str(&groups.date_format);
    regex_string.push_str(r") (?P<id>");
    regex_string.push_str(&groups.id);
    regex_string.push_str(r") (?P<name>)");
    regex_string.push_str(&groups.name);
    regex_string.push_str(r"(?P<output>");
    regex_string.push_str(&groups.output);
    regex_string.push_str(r")");

    let regex = Regex::new(&*regex_string).unwrap();

    for line in log_str.lines() {
        if regex.is_match(&line) {
            counter += 1;
        }
    }

    let nr_of_lines = log_str.lines().count();
    println!("File contents: {} lines", nr_of_lines);
    println!("Lines matched: {} with {}", counter, regex_string);
}

fn get_string_from_file(file_nae: &str) -> String {
    let mut schema = File::open(file_nae).expect("Unable to open file");
    let mut schema_str = String::new();
    schema.read_to_string(&mut schema_str).expect("Unable to read file");
    schema_str
}
