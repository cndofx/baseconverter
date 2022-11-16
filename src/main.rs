use baseconverter::parse;
use clap::Parser;
use cli::Cli;
use error::BResult;

mod cli;
mod error;

fn main() -> BResult<()> {
    println!("Hello, world!");
    let cli = Cli::parse();
    let parsed = parse(&cli.input_value, cli.input_base);

    Ok(())
}
