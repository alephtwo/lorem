use clap::Parser;

fn main() {
    let args = Cli::parse();
    match args.result_type.as_str() {
        "characters" => print!("{}", generate(args.number, " ", "")),
        "words" => print!("{}", generate(args.number, " ", " ")),
        "paragraphs" => print_paragraphs(args.number),
        _ => eprintln!("how on earth did you get here?"),
    }
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

fn get_paragraphs() -> Vec<String> {
    serde_json::from_str(include_str!("paragraphs.json")).unwrap()
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(required = true, value_parser = clap::value_parser!(u16).range(1..50))]
    number: u16,

    #[arg(required = true, value_name = "TYPE", value_parser = ["characters", "words", "paragraphs"])]
    result_type: String,
}
