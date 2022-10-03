use clap::{Command, Arg, ArgAction, value_parser};

fn main() {
    let _matches = Command::new("leet")
                                .version("1.0")
                                .author("Ethan Cox (ethanrcox@protonmail.com")
                                .about("leet - A l33tsp34k translator")
                                .arg(
                                    Arg::new("level")
                                    .short('l')
                                    .long("level")
                                    .help("determines the level of intensity")
                                    .num_args(1)
                                    .default_value("1")
                                    .value_parser(value_parser!(u8))
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

    
}