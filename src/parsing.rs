use std::fs::OpenOptions;
use std::io::BufWriter;
use clap::{command, Command, arg, Arg, ArgMatches};
use anyhow::Result;
use crate::api::request::ConfigData;
use crate::error::error::Error;

pub fn parse() -> Result<Option<ArgMatches>> {
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

    if let Some(("auth", auth_args)) = args.subcommand() {
        return match auth_args.get_one::<String>("TOKEN") {
            Some(token) => {
                authenticate(token)
            }
            None => {
                println!("{}", Error::new("A token needs to be provided", None));
                std::process::exit(1)
            }
        }
    }

    return Ok(Option::from(args))
}

pub fn authenticate(token: &str) -> Result<Option<ArgMatches>>{
    let mut config: ConfigData = ConfigData::new()?;
    config.token = token.to_owned();

    let file = OpenOptions::new().write(true).truncate(true).open("config.json")?;
    serde_json::to_writer_pretty(BufWriter::new(file), &config)?;

    println!("Added authentication token: {}", token);
    return Ok(None)
}