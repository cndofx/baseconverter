use error::BResult;

mod error;

pub enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

pub fn parse(input: &str, from: Base) -> BResult<i64> {
    let trimmed = match from {
        Base::Binary => input.strip_prefix("0b").unwrap_or(input),
        Base::Octal => input.strip_prefix("0o").unwrap_or(input),
        Base::Hexadecimal => input.strip_prefix("0x").unwrap_or(input),
        Base::Decimal => input,
    };
    let value = match from {
        Base::Binary => i64::from_str_radix(trimmed, 2)?,
        Base::Octal => i64::from_str_radix(trimmed, 8)?,
        Base::Hexadecimal => i64::from_str_radix(trimmed, 16)?,
        Base::Decimal => i64::from_str_radix(trimmed, 10)?,
    };
    Ok(value)
}

pub fn convert(input: i64, to: Base) -> String {
    match to {
        Base::Binary => format!("0b{:b}", input),
        Base::Octal => format!("0o{:o}", input),
        Base::Hexadecimal => format!("0x{:X}", input),
        Base::Decimal => format!("{}", input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("5001", Base::Decimal).unwrap(), 5001);
        assert_eq!(parse("0xF", Base::Hexadecimal).unwrap(), 15);
        assert_eq!(parse("1234", Base::Hexadecimal).unwrap(), 4660);
        assert_eq!(parse("0xDEADBEEF", Base::Hexadecimal).unwrap(), 3735928559);
        assert_eq!(parse("0b10", Base::Binary).unwrap(), 2);
        assert_eq!(parse("0o15", Base::Octal).unwrap(), 13);
    }

    #[test]
    fn test_convert() {
        assert_eq!(convert(5001, Base::Decimal), "5001");
        assert_eq!(convert(15, Base::Hexadecimal), "0xF");
        assert_eq!(convert(4660, Base::Hexadecimal), "0x1234");
        assert_eq!(convert(3735928559, Base::Hexadecimal), "0xDEADBEEF");
        assert_eq!(convert(2, Base::Binary), "0b10");
        assert_eq!(convert(13, Base::Octal), "0o15");
    }
}
