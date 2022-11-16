use crate::error::BResult;

pub mod error;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Base {
    #[clap(name = "bin")]
    Binary,
    #[clap(name = "oct")]
    Octal,
    #[clap(name = "dec")]
    Decimal,
    #[clap(name = "hex")]
    Hexadecimal,
}

impl std::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Base::Binary => write!(f, "binary"),
            Base::Octal => write!(f, "octal"),
            Base::Decimal => write!(f, "decimal"),
            Base::Hexadecimal => write!(f, "hexadecimal"),
        }
    }
}

pub fn strip_input<'a>(input: &'a str, from: Base) -> &'a str {
    match from {
        Base::Binary => input.strip_prefix("0b").unwrap_or(input),
        Base::Octal => input.strip_prefix("0o").unwrap_or(input),
        Base::Hexadecimal => input.strip_prefix("0x").unwrap_or(input),
        Base::Decimal => input,
    } 
}

pub fn parse_input(input: &str, from: Base) -> BResult<i64> {
    let stripped = strip_input(input, from);
    let value = match from {
        Base::Binary => i64::from_str_radix(stripped, 2),
        Base::Octal => i64::from_str_radix(stripped, 8),
        Base::Hexadecimal => i64::from_str_radix(stripped, 16),
        Base::Decimal => i64::from_str_radix(stripped, 10),
    };
    value.map_err(|_| error::BaseConverterError::ParseError { input: input.to_string(), base: from, label_pos: (0,input.len()) })
}

pub fn format_output(output: i64, to: Base) ->  String {
    match to {
        Base::Binary => format!("0b{:b}", output),
        Base::Octal => format!("0o{:o}", output),
        Base::Hexadecimal => format!("0x{:X}", output),
        Base::Decimal => format!("{}", output),
    }
}

pub fn convert(input: &str, from: Base, to: Base) -> BResult<(String, String)> {
    let parsed = parse_input(input, from)?;
    let formatted_input = format_output(parsed, from);
    let formatted_output = format_output(parsed, to);
    Ok((formatted_input, formatted_output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_input("5001", Base::Decimal).unwrap(), 5001);
        assert_eq!(parse_input("0xF", Base::Hexadecimal).unwrap(), 15);
        assert_eq!(parse_input("1234", Base::Hexadecimal).unwrap(), 4660);
        assert_eq!(parse_input("0xDEADBEEF", Base::Hexadecimal).unwrap(), 3735928559);
        assert_eq!(parse_input("0b10", Base::Binary).unwrap(), 2);
        assert_eq!(parse_input("0o15", Base::Octal).unwrap(), 13);
    }

    #[test]
    fn test_convert() {
        assert_eq!(convert("5001", Base::Decimal, Base::Decimal).unwrap().1, "5001");
        assert_eq!(convert("15", Base::Decimal, Base::Hexadecimal).unwrap().1, "0xF");
        assert_eq!(convert("4660", Base::Decimal, Base::Hexadecimal).unwrap().1, "0x1234");
        assert_eq!(convert("3735928559", Base::Decimal, Base::Hexadecimal).unwrap().1, "0xDEADBEEF");
        assert_eq!(convert("2", Base::Decimal, Base::Binary).unwrap().1, "0b10");
        assert_eq!(convert("13", Base::Decimal, Base::Octal).unwrap().1, "0o15");
    }
}
