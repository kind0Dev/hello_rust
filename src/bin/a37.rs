// Topic: TryFrom/TryInto
//
// Summary:
// * A library is needed for an application to convert hex color codes
//   into their component color values (red, green, and blue). Hex color codes
//   consist of a hash symbol followed by six hex digits. Every two hex digits
//   represent a color component in the order of red, green, blue.
//
//   Example hex color codes:
//    #ffffff -> Rgb(255, 255, 255)
//    #001122 -> Rgb(0, 17, 34)
//
// Requirements:
// * Create a program to convert a hex code (as &str) into an Rgb struct
// * Implement TryFrom to perform the conversion
// * Utilize the question mark operator in your implementation
//
// Notes:
// * See the `from_str_radix` function in the stdlib docs for `u8`
//   to convert hex digits to `u8`
//   * Hex digits use a radix value of 16
// * Utilize the `thiserror` crate for your error type
// * Run `cargo test --bin a37` to test your implementation

use thiserror::Error;
use std::convert::TryFrom;

#[derive(Debug, Error)]
enum RgbError {
    #[error("missing # symbol")]
    MissingHash,
    #[error("failed to parse hex to int: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("hex length to short or to long")]
    LengthError,
}

#[derive(Debug, Eq, PartialEq)]
struct Rgb(u8, u8, u8);

impl TryFrom<&str> for Rgb {
    type Error = RgbError;

    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        if hex.len() != 7{
            return Err(RgbError::LengthError)
        } 
         
        if hex.chars().nth(0).unwrap() != '#' {
            return Err(RgbError::MissingHash)
        }

        let (r,g,b) = (u8::from_str_radix(&hex[1..=2], 16)?, u8::from_str_radix(&hex[3..=4], 16)?, u8::from_str_radix(&hex[5..=6], 16)?);
        return Ok(Self(r, g, b));



    }
    
}

fn main() {
    // Use `cargo test --bin a37` to test your implementation
}

#[cfg(test)]
mod test {
    use super::Rgb;
    use std::convert::TryFrom;

    #[test]
    fn converts_valid_hex_color() {
        let expected = Rgb(0, 204, 102);
        let actual = Rgb::try_from("#00cc66");
        assert_eq!(
            actual.is_ok(),
            true,
            "valid hex code should be converted to Rgb"
        );
        assert_eq!(actual.unwrap(), expected, "wrong Rgb value");
    }

    #[test]
    fn fails_on_invalid_hex_digits() {
        assert_eq!(
            Rgb::try_from("#0011yy").is_err(),
            true,
            "should be an error with invalid hex color"
        );
    }

    #[test]
    fn fails_when_missing_hash() {
        assert_eq!(
            Rgb::try_from("001100").is_err(),
            true,
            "should be an error when missing hash symbol"
        );
    }

    #[test]
    fn fails_when_missing_color_components() {
        assert_eq!(
            Rgb::try_from("#0011f").is_err(),
            true,
            "should be an error when missing one or more color components"
        );
    }

    #[test]
    fn fails_with_too_many_color_components() {
        assert_eq!(
            Rgb::try_from("#0011ffa").is_err(),
            true,
            "should be an error when too many color components are provided"
        );
    }
}
