use clap::{arg, Command, value_parser, ArgAction};

fn main() {

    let matches = Command::new("echor")
        .author("someone")
        .about("rust echo")
        .version("0.1.0")
        .arg(arg!(text: <TEXT> "Input text")
             .action(ArgAction::Append)
             .value_parser(value_parser!(String)))
        .arg(arg!(omit_newline: -n "Do not print newline")
             .value_parser(value_parser!(bool)))
        .get_matches();


    // println!("{:#?}", matches)

    let texts: Vec<String> = matches.get_many::<String>("text")
        .expect("provide a 'input text'")
        .map(|s| s.clone())
        .collect();

    let no_newline = matches.get_one::<bool>("omit_newline").unwrap();
    if *no_newline {
        print!("{}", texts.join(" "));
    } else {
        println!("{}", texts.join(" "));
    }
}
