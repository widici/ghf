use clap::{command, Arg, ArgMatches};

pub fn parse() -> Result<ArgMatches, clap::Error> {
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
                .value_name("COLOR")
                .long("color")
                .short('c'),
        )
        .get_matches();

    return Ok ( args )
}