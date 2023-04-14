use clap::{App, Arg, ArgMatches};

pub fn parse() -> Result<ArgMatches, clap::Error> {
    return Ok (
        App::new("ghfetch")
        .version("1.0")
        .author("widici")
        .about("Cli program that fetches and displays Github users data")
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
                .value_name("COLOR")
                .long("color")
                .short('c'),
        )
        .get_matches()
    )
}