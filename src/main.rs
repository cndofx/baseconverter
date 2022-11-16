use clap::Parser;
use cli::Cli;

use baseconverter::error::BResult;
use baseconverter::{convert, parse};

mod cli;

fn main() -> BResult<()> {
    let cli = Cli::parse();
    let input = parse(&cli.input_value, cli.input_base)?;
    let output = convert(input, cli.output_base);
    println!("{}", output);
    Ok(())
}
