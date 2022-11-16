use clap::Parser;
use cli::Cli;

use baseconverter::convert;

mod cli;

fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    let (input, output) = convert(&cli.input_value, cli.input_base, cli.output_base)?;
    println!("Converted {} ({}) to {} ({})", input, &cli.input_base, output, &cli.output_base);
    Ok(())
}
