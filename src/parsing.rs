use clap::{command, arg, Arg, ArgMatches};
use anyhow::Result;
use clap::Command;

pub fn parse() -> Result<ArgMatches> {
    let args = command!()
        .arg(arg!([NAME]))
        .arg(
            Arg::new("color")
                .help("Optional color of the text of the fetched data")
                .required(false)
                .value_parser(["black", "red", "green", "yellow", "blue", "magenta", "purple", "cyan", "white"])
                .value_name("COLOR")
                .long("color")
                .short('c')
        )
        .subcommand(
            Command::new("auth")
                .about("Authenticates to the GitHub api with a authentication token")
                .arg(arg!([TOKEN]))
        )
        .get_matches();

    return Ok(args)
}