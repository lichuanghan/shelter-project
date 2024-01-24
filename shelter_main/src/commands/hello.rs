use clap::{ArgMatches, Command};

pub fn configure() -> Command{
    Command::new("hello").about("hello world!")
}

pub fn handle(matches:&ArgMatches)->anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("hello") {
        println!("hello world!")
    }

    Ok(())

}