// This module is a giant mess. Hexadecimal floats suck to parse and everyone seems to do it differently.
// I'm not sure if this is the best way to do it, but it works.
//
// Thanks to Julia Scheaffer for coming up with the code this is based on.
// You will be remembered for your contribution to society.

//! # Hexponent
//!
//! Hexponent is a hexadecimal literal parser for Rust based on the C11
//! specification section [6.4.4.2](http://port70.net/~nsz/c/c11/n1570.html#6.4.4.2).
//!
//! ```rust
//! use hexponent::FloatLiteral;
//! let float_repr: FloatLiteral = "0x3.4".parse().unwrap();
//! let value = float_repr.convert::<f32>().inner();
//! assert_eq!(value, 3.25);
//! ```
//! Hexponent has a minimum supported rust version of 1.34.
//!
//! ## Features
//! - No dependencies
//! - Non-UTF-8 parser
//! - Precision warnings
//! - `no_std` support (MSRV 1.36.0)
//!
//! ## Differences from the specification
//! There are two places where hexponent differs from the C11 specificaiton.
//! - An exponent is not required. (`0x1.2` is allowed)
//! - `floating-suffix` is *not* parsed. (`0x1p4l` is not allowed)
//!
//! ## `no_std` support
//! `no_std` support can be enabled by disabling the default `std` feature for
//! hexponent in your `Cargo.toml`.
//! ```toml
//! hexponent = {version = "0.2", default-features = false}
//! ```
//! `no_std` support is only possible in rustc version 1.36.0 and higher.
//!
//! Disabling the `std` feature currently only disables the `std::error::Error`
//! implementation for `ParseError`.

use core::fmt;

#[derive(Debug)]
/// Indicates the preicsision of a conversion
pub enum ConversionResult<T> {
    /// The conversion was precise and the result represents the original exactly.
    Precise(T),

    // TODO: I should be able to calculate how imprecise the conversion is too,
    // which might be useful. This also might allow some subnormal numbers to be
    // returned as precise results.
    /// The conversion was imprecise and the result is as close to the original
    /// as possible.
    Imprecise(T),
}

impl<T> ConversionResult<T> {
    /// Convert the result to it's contained type.
    pub fn inner(self) -> T {
        match self {
            ConversionResult::Precise(f) => f,
            ConversionResult::Imprecise(f) => f,
        }
    }
}

/// Error type for parsing hexadecimal literals.
///
/// See the [`ParseErrorKind`](enum.ParseErrorKind.html) documentation for more
/// details about the kinds of errors and examples.
///
/// `ParseError` only implements `std::error::Error` when the `std` feature is
/// enabled.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ParseError {
    /// Kind of error
    pub kind: ParseErrorKind,
    /// Approximate index of the error in the source data. This will always be
    /// an index to the source, except for when something is expected and
    /// nothing is found, in this case, `index` will be the length of the input.
    pub index: usize,
}

/// Kind of parsing error.
///
/// Used in [`ParseError`](struct.ParseError.html)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParseErrorKind {
    /// No prefix was found. Hexadecimal literals must start with a "0x" or "0X"
    /// prefix.
    ///
    /// Example: `0.F`
    MissingPrefix,
    /// No digits were found. Hexadecimals literals must have digits before or
    /// after the decimal point.
    ///
    /// Example: `0x.` `0x.p1`
    MissingDigits,
    /// Hexadecimal literals with a "p" or "P" to indicate an float must have
    /// an exponent.
    ///
    /// Example: `0xb.0p` `0x1p-`
    MissingExponent,
    /// The exponent of a hexidecimal literal must fit into a signed 32-bit
    /// integer.
    ///
    /// Example: `0x1p3000000000`
    ExponentOverflow,
    /// The end of the literal was expected, but more bytes were found.
    ///
    /// Example: `0x1.g`
    MissingEnd,
}

impl ParseErrorKind {
    fn at(self, index: usize) -> ParseError {
        ParseError { kind: self, index }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ParseErrorKind::MissingPrefix => write!(f, "literal must have hex prefix"),
            ParseErrorKind::MissingDigits => write!(f, "literal must have digits"),
            ParseErrorKind::MissingExponent => write!(f, "exponent not present"),
            ParseErrorKind::ExponentOverflow => write!(f, "exponent too large to fit in integer"),
            ParseErrorKind::MissingEnd => {
                write!(f, "extra bytes were found at the end of float literal")
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// Represents a floating point literal
///
/// This struct is a representation of the text, that can be used to convert to
/// both single- and double-precision floats.
///
/// `FloatLiteral` is not `Copy`-able because it contains a vector of the
/// digits from the source data.
#[derive(Debug, Clone)]
pub struct FloatLiteral {
    is_positive: bool,
    // These are the values of the digits, not the digits in ascii form.
    digits: Vec<u8>,
    decimal_offset: i32,
    exponent: i32,
}

/// Get the byte index of the start of `sub_slice` in `master_slice`
fn get_cursed_index(master_slice: &[u8], sub_slice: &[u8]) -> usize {
    (sub_slice.as_ptr() as usize).saturating_sub(master_slice.as_ptr() as usize)
}

impl FloatLiteral {
    /// Convert the `self` to an `f32` or `f64` and return the precision of the
    /// conversion.
    pub fn convert<F: FPFormat>(self) -> ConversionResult<F> {
        F::from_literal(self)
    }

    /// Parse a slice of bytes into a `FloatLiteral`.
    ///
    /// This is based on hexadecimal floating constants in the C11 specification,
    /// section [6.4.4.2](http://port70.net/~nsz/c/c11/n1570.html#6.4.4.2).
    pub fn from_bytes(data: &[u8]) -> Result<FloatLiteral, ParseError> {
        let original_data = data;

        let (is_positive, data) = match data.get(0) {
            Some(b'+') => (true, &data[1..]),
            Some(b'-') => (false, &data[1..]),
            _ => (true, data),
        };

        let data = match data.get(0..2) {
            Some(b"0X") | Some(b"0x") => &data[2..],
            _ => return Err(ParseErrorKind::MissingPrefix.at(0)),
        };

        let (ipart, data) = consume_hex_digits(data);

        let (fpart, data): (&[_], _) = if data.get(0) == Some(&b'.') {
            let (fpart, data) = consume_hex_digits(&data[1..]);
            (fpart, data)
        } else {
            (b"", data)
        };

        // Must have digits before or after the decimal point.
        if fpart.is_empty() && ipart.is_empty() {
            return Err(ParseErrorKind::MissingDigits.at(get_cursed_index(original_data, data)));
        }

        let (exponent, data) = match data.get(0) {
            Some(b'P') | Some(b'p') => {
                let data = &data[1..];

                let sign_offset = match data.get(0) {
                    Some(b'+') | Some(b'-') => 1,
                    _ => 0,
                };

                let exponent_digits_offset = data[sign_offset..]
                    .iter()
                    .position(|&b| match b {
                        b'0'..=b'9' => false,
                        _ => true,
                    })
                    .unwrap_or_else(|| data[sign_offset..].len());

                if exponent_digits_offset == 0 {
                    return Err(
                        ParseErrorKind::MissingExponent.at(get_cursed_index(original_data, data))
                    );
                }

                // The exponent should always contain valid utf-8 beacuse it
                // consumes a sign, and base-10 digits.
                // TODO: Maybe make this uft8 conversion unchecked. It should be
                // good, but I also don't want unsafe code.
                let exponent: i32 =
                    core::str::from_utf8(&data[..sign_offset + exponent_digits_offset])
                        .expect("exponent did not contain valid utf-8")
                        .parse()
                        .map_err(|_| {
                            ParseErrorKind::ExponentOverflow
                                .at(get_cursed_index(original_data, data))
                        })?;

                (exponent, &data[sign_offset + exponent_digits_offset..])
            }
            _ => (0, data),
        };

        if !data.is_empty() {
            return Err(ParseErrorKind::MissingEnd.at(get_cursed_index(original_data, data)));
        }

        let mut raw_digits = ipart.to_vec();
        raw_digits.extend_from_slice(fpart);

        let first_digit = raw_digits.iter().position(|&d| d != b'0');

        let (digits, decimal_offset) = if let Some(first_digit) = first_digit {
            // Unwrap is safe because there is at least one digit.
            let last_digit = raw_digits.iter().rposition(|&d| d != b'0').unwrap();
            let decimal_offset = (ipart.len() as i32) - (first_digit as i32);

            // Trim off the leading zeros
            raw_digits.truncate(last_digit + 1);
            // Trim off the trailing zeros
            raw_digits.drain(..first_digit);

            // Convert all the digits from ascii to their values.
            for item in raw_digits.iter_mut() {
                *item = hex_digit_to_int(*item).unwrap();
            }

            (raw_digits, decimal_offset)
        } else {
            (Vec::new(), 0)
        };

        Ok(FloatLiteral {
            is_positive,
            digits,
            decimal_offset,
            exponent,
        })
    }
}

impl core::str::FromStr for FloatLiteral {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<FloatLiteral, ParseError> {
        FloatLiteral::from_bytes(s.as_bytes())
    }
}

impl From<FloatLiteral> for f32 {
    fn from(literal: FloatLiteral) -> f32 {
        literal.convert().inner()
    }
}

impl From<FloatLiteral> for f64 {
    fn from(literal: FloatLiteral) -> f64 {
        literal.convert().inner()
    }
}

fn hex_digit_to_int(digit: u8) -> Option<u8> {
    match digit {
        b'0' => Some(0x0),
        b'1' => Some(0x1),
        b'2' => Some(0x2),
        b'3' => Some(0x3),
        b'4' => Some(0x4),
        b'5' => Some(0x5),
        b'6' => Some(0x6),
        b'7' => Some(0x7),
        b'8' => Some(0x8),
        b'9' => Some(0x9),
        b'a' | b'A' => Some(0xa),
        b'b' | b'B' => Some(0xb),
        b'c' | b'C' => Some(0xc),
        b'd' | b'D' => Some(0xd),
        b'e' | b'E' => Some(0xe),
        b'f' | b'F' => Some(0xf),
        _ => None,
    }
}

fn consume_hex_digits(data: &[u8]) -> (&[u8], &[u8]) {
    let i = data
        .iter()
        .position(|&b| hex_digit_to_int(b).is_none())
        .unwrap_or_else(|| data.len());

    data.split_at(i)
}

use core::ops;

macro_rules! impl_fpformat {
    ($fp_type:ty, $bits_type:ty, $exponent_bits: literal, $mantissa_bits: literal, $from_bits: expr, $infinity: expr, $max_exp: expr, $min_exp: expr) => {
        impl FPFormat for $fp_type {
            fn from_literal(literal: FloatLiteral) -> ConversionResult<$fp_type> {
                const EXPONENT_BITS: u32 = $exponent_bits;
                const MANTISSA_BITS: u32 = $mantissa_bits;

                const TOTAL_BITS: u32 = 1 + EXPONENT_BITS + MANTISSA_BITS;

                // The spec always gives an exponent bias that follows this formula.
                const EXPONENT_BIAS: u32 = (1 << (EXPONENT_BITS - 1)) - 1;

                // 4 bits for each hexadecimal offset
                let mut exponent_offset: i32 = literal.decimal_offset * 4;

                // If there were all
                if literal.digits.is_empty() {
                    return ConversionResult::Precise(0.0);
                }

                // This code is a work of art.
                let mut was_truncated = false;
                let mut mantissa_result: $bits_type = 0;
                for (index, digit) in literal.digits.iter().enumerate() {
                    if index as u32 * 4 > MANTISSA_BITS {
                        was_truncated = true;
                        break;
                    }
                    let mut digit_value = *digit as $bits_type;
                    digit_value <<= TOTAL_BITS - (index as u32 + 1) * 4;
                    mantissa_result |= digit_value;
                }
                let leading_zeros = mantissa_result.leading_zeros();
                exponent_offset -= leading_zeros as i32 + 1;
                mantissa_result <<= leading_zeros + 1;
                mantissa_result >>= TOTAL_BITS - MANTISSA_BITS;

                let final_exponent = exponent_offset + literal.exponent;

                // Check for underflows
                if final_exponent < $min_exp - 1 {
                    // TODO: Implement subnormal numbers.
                    if literal.is_positive {
                        return ConversionResult::Imprecise(0.0);
                    } else {
                        return ConversionResult::Imprecise(-0.0);
                    };
                }

                // Check for overflows
                if final_exponent > $max_exp - 1 {
                    if literal.is_positive {
                        return ConversionResult::Imprecise($infinity);
                    } else {
                        return ConversionResult::Imprecise(-$infinity);
                    };
                }

                let exponent_result: $bits_type =
                    ((final_exponent + EXPONENT_BIAS as i32) as $bits_type) << MANTISSA_BITS;

                let sign_result: $bits_type =
                    (!literal.is_positive as $bits_type) << (MANTISSA_BITS + EXPONENT_BITS);

                let float_value = $from_bits(sign_result | exponent_result | mantissa_result);

                if was_truncated {
                    ConversionResult::Imprecise(float_value)
                } else {
                    ConversionResult::Precise(float_value)
                }

                // // This might be a bit faster.
                // let mut final_result = !literal.is_positive as $bits_type;
                // final_result <<= EXPONENT_BITS;
                // final_result |= (final_exponent + EXPONENT_BIAS as i32) as $bits_type;
                // final_result <<= MANTISSA_BITS;
                // final_result |= mantissa_result;
                // ConversionResult::Precise($from_bits(final_result))
            }
        }
    };
}

/// Trait to describe conversion to floating point formats.
pub trait FPFormat: ops::Neg<Output = Self> + Sized + Copy {
    /// Convert a literal to this format. This is a hack so that we can use
    /// a macro to implement conversions.
    fn from_literal(literal: FloatLiteral) -> ConversionResult<Self>;
}

impl_fpformat!(
    f32,
    u32,
    8,
    23,
    f32::from_bits,
    core::f32::INFINITY,
    core::f32::MAX_EXP,
    core::f32::MIN_EXP
);
impl_fpformat!(
    f64,
    u64,
    11,
    52,
    f64::from_bits,
    core::f64::INFINITY,
    core::f64::MAX_EXP,
    core::f64::MIN_EXP
);
