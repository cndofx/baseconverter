use baseconverter::Base;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub input_value: String,
    #[arg(short, long = "input", value_enum, default_value_t = Base::Decimal)]
    pub input_base: Base,
    #[arg(short, long = "output", value_enum, default_value_t = Base::Decimal)]
    pub output_base: Base,
}