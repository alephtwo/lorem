extern crate serde_json;

#[macro_use]
extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("number")
                .value_name("number")
                .validator(is_number)
                .required(true),
        )
        .arg(
            Arg::with_name("type")
                .value_name("type")
                .possible_values(&["characters", "words", "paragraphs"])
                .required(true),
        )
        .get_matches();

    let number = matches.value_of("number").unwrap().parse::<usize>().unwrap();
    let result_type = matches.value_of("type").unwrap();

    match result_type {
        "characters" => print!("{}", generate(number, " ", "")),
        "words" => print!("{}", generate(number, " ", " ")),
        "paragraphs" => print_paragraphs(number),
        _ => eprintln!("how on earth did you get here?"),
    }
}

fn is_number(val: String) -> Result<(), String> {
    match val.parse::<i32>() {
        Ok(_val) => Ok(()),
        Err(_err) => Err(format!("{} is not a number", val).to_owned()),
    }
}

fn get_paragraphs() -> Vec<String> {
    serde_json::from_str(include_str!("paragraphs.json")).unwrap()
}

fn generate(number: usize, joiner: &str, splitter: &str) -> String {
    get_paragraphs()
        .join(joiner)
        .split(splitter)
        .map(str::to_owned)
        .filter(|t| !t.is_empty())
        .collect::<Vec<String>>()
        .iter()
        .cycle()
        .take(number)
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join(splitter)
}

fn print_paragraphs(number: usize) {
    let paragraphs = get_paragraphs()
        .iter()
        .cycle()
        .take(number)
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join("\n\n");

    print!("{}", paragraphs);
}
