use std::str::FromStr;

/// Guess the string radix based on the first two numbers
/// Returns `(radix, str_with_leading_radix)`.
/// # Example
/// ```
///    assert_eq!(guess_str_radix("0x10"), Some(16, "10"))
/// ```
pub fn guess_str_radix(s: &str) -> Option<(u32, &str)> {
    let mut chars = s.chars();

    match chars.next()? {
        '0' => {
            let after_zero = chars.as_str();
            match chars.next() {
                Some('o') | Some('O') => Some((8, chars.as_str())),
                Some('b') | Some('B') => Some((2, chars.as_str())),
                Some('x') | Some('X') => Some((16, chars.as_str())),
                None => Some((10, s)),
                Some(mut c)  => {
                    loop {
                        if c.to_digit(10)? > 8 {
                            return Some((10, after_zero))
                        }
                        c = match chars.next() {
                            Some(c) => c,
                            None => break,
                        };
                    }
                    Some((8, after_zero))
                }
            }},
        c if c.is_numeric() => Some((10, s)),
        _ => None
    }
}


pub struct Number {
    num: f64
}
pub struct BigInt {
    bigint: num_bigint::BigInt
}
pub enum BigIntError {
    Empty,
    InvalidDigit(u32),
    BadRadix
}
impl BigInt {
    pub fn from_str_radix(s: &str, radix: u32) -> Result<BigInt, BigIntError> {
        if s.is_empty() {
            return Err(BigIntError::Empty)
        }
        match num_bigint::BigInt::from_str_radix(s, radix) {
            Ok(i) => Ok(BigInt {
                bigint: i
            }),
            Err(e) => Err(BigIntError::InvalidDigit(radix))
        }
    }

}
impl FromStr for BigInt {
    type Err = BigIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match guess_str_radix(s) {
            Some((radix, rest)) => BigInt::from_str_radix(rest, radix),
            None => Err(BigIntError::BadRadix)
        }
    }
}