use clap::{Parser, Subcommand, ValueEnum};
use anyhow::{Result, anyhow};
use url::Url;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    extract_command: String,
    target: String,
}


fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let output = cli.extract_command;
    let url = Url::parse(&cli.target)?;

    let output = output.replace("path", &url.path()[1..]);
    println!("{}", output);
    Ok(())
}