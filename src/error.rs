use std::num::ParseIntError;

use miette::Diagnostic;
use thiserror::Error;

use crate::Base;

pub type BResult<T> = Result<T, BaseConverterError>;

#[derive(Error, Debug, Diagnostic)]
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

    #[error("number is too large")]
    #[diagnostic(
        code(baseconverter::input_overflow_error),
        help("ensure your input value fits between -(2^63) and (2^63 - 1)")
    )]
    OverflowError {
        #[source_code]
        input: String,
        #[label("ensure this number fits within the limits")]
        label_pos: (usize, usize),
    },

    #[error("unknown error")]
    #[diagnostic(
        code(baseconverter::unknown_error),
        help("unknown error, please check your input")
    )]
    UnknownError,
}

impl BaseConverterError {
    pub fn from_parse_error(e: ParseIntError, input: &str, base: Base) -> BaseConverterError {
        match e.kind() {
            std::num::IntErrorKind::InvalidDigit => BaseConverterError::ParseError {
                input: input.to_string(),
                label_pos: (0, input.len()),
                base: base,
            },
            std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => {
                BaseConverterError::OverflowError {
                    input: input.to_string(),
                    label_pos: (0, input.len()),
                }
            }

            _ => BaseConverterError::UnknownError,
        }
    }
}
