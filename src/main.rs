mod leetspeak;
mod translation_tables;

use clap::{Command, Arg, ArgAction, value_parser};

fn main() {
    let matches = Command::new("leet")
                                .version("1.0")
                                .author("Ethan Cox (ethanrcox@protonmail.com")
                                .about("leet - a l33tsp34k translator")
                                .arg(
                                    Arg::new("level")
                                    .short('l')
                                    .long("level")
                                    .help("determines the level of intensity")
                                    .num_args(1)
                                    .default_value("1")
                                    .value_parser(
                                        value_parser!(u8)
                                        .range(1..=3)
                                    )
                                    .action(ArgAction::Set)
                                )
                                .arg(
                                    Arg::new("text")
                                    .index(1)
                                    .required(true)
                                    .help("the text to translate as a single string")
                                    .action(ArgAction::Set)
                                )
                                .get_matches();

    let text = matches.get_one::<String>("text").expect("text is required");
    let level = matches.get_one::<u8>("level").expect("level has a default value");
    println!("{}", leetspeak::translate(text, level));
}