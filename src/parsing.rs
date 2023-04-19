use clap::{command, Arg, ArgMatches};

pub fn parse() -> Result<ArgMatches, Box<dyn std::error::Error>> {
    let args = command!()
        .arg(
            Arg::new("name")
                .help("Username of the Github user")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("color")
                .help("Optional color of the text of the fetched data")
                .required(false)
                .value_parser(["black", "red", "green", "yellow", "blue", "magenta", "purple", "cyan", "white"])
                .value_name("COLOR")
                .long("color")
                .short('c'),
        )
        .get_matches();

    return Ok(args)
}