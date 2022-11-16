use miette::Diagnostic;
use thiserror::Error;

use crate::Base;

pub type BResult<T> = Result<T, BaseConverterError>;

#[derive(Error, Debug, Diagnostic)]
// #[error("#[error()] tag example")]
// #[diagnostic(help("ensure your input value contains only valid characters for the selected base"))]
pub enum BaseConverterError {
    #[error("unable to parse user input")]
    #[diagnostic(
        code(baseconverter::input_parsing_error),
        help("ensure your input value ({input}) contains only valid characters for {base}")
    )]
    ParseError {
        #[source_code]
        input: String,
        #[label("ensure this is valid {base}")]
        label_pos: (usize, usize),
        base: Base,

    },
}
