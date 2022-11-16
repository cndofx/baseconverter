use miette::Diagnostic;
use thiserror::Error;

use crate::Base;

pub type BResult<T> = Result<T, BaseConverterError>;

#[derive(Error, Debug, Diagnostic)]
// #[error("#[error()] tag example")]
// #[diagnostic(help("ensure your input value contains only valid characters for the selected base"))]
pub enum BaseConverterError {
    #[error("input parsing error")]
    #[diagnostic(
        code(baseconverter::input_parsing_error),
        help("ensure your input value '{input}' contains only valid characters for {base}")
    )]
    ParseError {
        input: String,
        base: Base,
    },
}
