use thiserror::Error;

pub type BResult<T> = Result<T, BaseConverterError>;

#[derive(Error, Debug)]
pub enum BaseConverterError {
    #[error("input parsing error")]
    ParseError(#[from] std::num::ParseIntError),
}