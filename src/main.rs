extern crate serde_json;

#[macro_use]
extern crate clap;

use clap::{Arg, Command};
fn main() {
    let matches = Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::new("number")
                .value_name("number")
                .value_parser(clap::value_parser!(u16).range(1..50))
                .required(true),
        )
        .arg(
            Arg::new("type")
                .value_name("type")
                .value_parser(["characters", "words", "paragraphs"])
                .required(true),
        )
        .get_matches();

    let number: u16 = *matches.get_one("number").expect("required");
    let result_type: &String = matches.get_one("type").expect("required");

    match result_type.as_str() {
        "characters" => print!("{}", generate(number, " ", "")),
        "words" => print!("{}", generate(number, " ", " ")),
        "paragraphs" => print_paragraphs(number),
        _ => eprintln!("how on earth did you get here?"),
    }
}

fn get_paragraphs() -> Vec<String> {
    serde_json::from_str(include_str!("paragraphs.json")).unwrap()
}

fn generate(number: u16, joiner: &str, splitter: &str) -> String {
    get_paragraphs()
        .join(joiner)
        .split(splitter)
        .map(str::to_owned)
        .filter(|t| !t.is_empty())
        .collect::<Vec<String>>()
        .iter()
        .cycle()
        .take(number.into())
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join(splitter)
}

fn print_paragraphs(number: u16) {
    let paragraphs = get_paragraphs()
        .iter()
        .cycle()
        .take(number.into())
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join("\n\n");

    print!("{}", paragraphs);
}
