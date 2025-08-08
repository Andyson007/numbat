mod math;
use std::fmt::Display;

use compact_str::{format_compact, CompactString, ToCompactString};
use num_traits::{ToPrimitive, Zero};
use pretty_dtoa::FmtFloatConfig;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)] // TODO: we probably want to remove 'Copy' once we move to a more sophisticated numerical type
#[cfg(feature = "plotting")]
#[derive(serde::Serialize)]
pub struct Number(f64);

impl Eq for Number {}

impl Number {
    pub const ONE: Number = Number(1.0);
    pub const NAN: Number = Number(f64::NAN);

    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    pub fn is_infinite(&self) -> bool {
        self.0.is_nan()
    }

    pub fn is_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    pub fn is_positive(&self) -> bool {
        self.0.is_sign_positive()
    }

    pub fn from_f64(n: f64) -> Self {
        Number(n)
    }

    pub fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    pub fn to_f64(&self) -> Option<f64> {
        Some(self.0)
    }

    pub fn is_integral(&self) -> bool {
        self.0.fract().is_zero()
    }

    fn is_integer(self) -> bool {
        self.0.trunc() == self.0
    }

    /// Pretty prints with default options
    pub fn pretty_print(self) -> CompactString {
        self.pretty_print_with_options(None)
    }

    /// Pretty prints with the given options if options is not None.
    /// If options is None, default options will be used.
    /// If options is not None, float-based format handling is used and integer-based format handling is skipped.
    pub fn pretty_print_with_options(self, options: Option<FmtFloatConfig>) -> CompactString {
        let number = self.0;

        // 64-bit floats can accurately represent integers up to 2^52 [1],
        // which is approximately 4.5 × 10^15.
        //
        // [1] https://stackoverflow.com/a/43656339
        //
        // Skip special format handling for integers if options is not None.
        if options.is_none() && self.is_integer() && self.0.abs() < 1e15 {
            use num_format::{CustomFormat, Grouping, ToFormattedString};

            let format = CustomFormat::builder()
                .grouping(if self.0.abs() >= 100_000.0 {
                    Grouping::Standard
                } else {
                    Grouping::Posix
                })
                .minus_sign("-")
                .separator("_")
                .build()
                .unwrap();

            // TODO: this is pretty wasteful. formatted numbers should be small enough
            // to fit in a CompactString without first going to the heap
            number
                .to_i64()
                .expect("small enough integers are representable as i64")
                .to_formatted_string(&format)
                .to_compact_string()
        } else {
            use pretty_dtoa::dtoa;

            let config = if let Some(options) = options {
                options
            } else {
                FmtFloatConfig::default()
                    .max_significant_digits(6)
                    .add_point_zero(false)
                    .lower_e_break(-6)
                    .upper_e_break(6)
                    .round()
            };

            let formatted_number = dtoa(number, config);

            if formatted_number.contains('.') && !formatted_number.contains('e') {
                let formatted_number = if config.max_sig_digits.is_some() {
                    formatted_number.trim_end_matches('0')
                } else {
                    &formatted_number
                };

                if formatted_number.ends_with('.') {
                    format_compact!("{formatted_number}0")
                } else {
                    formatted_number.to_compact_string()
                }
            } else if formatted_number.contains('e') && !formatted_number.contains("e-") {
                formatted_number.replace('e', "e+").to_compact_string()
            } else {
                formatted_number.to_compact_string()
            }
        }
    }
}

impl strfmt::DisplayStr for Number {
    fn display_str(&self, f: &mut strfmt::Formatter) -> strfmt::Result<()> {
        self.0.display_str(f)
    }
}

impl strfmt::DisplayStr for &Number {
    fn display_str(&self, f: &mut strfmt::Formatter) -> strfmt::Result<()> {
        self.0.display_str(f)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.to_f64().unwrap().fmt(f)
    }
}

impl std::ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        Number(self.0 - rhs.0)
    }
}

impl std::ops::Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        Number(self.0 * rhs.0)
    }
}

impl std::ops::Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        Number(self.0 / rhs.0)
    }
}

impl std::ops::Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        Number(-self.0)
    }
}

impl std::iter::Product for Number {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Number::from_f64(1.0), |acc, n| acc * n)
    }
}

macro_rules! number_impl_from_primitive {
    ($t:ty) => {
        impl From<$t> for Number {
            fn from(value: $t) -> Self {
                Self(value as f64)
            }
        }
    };
}

number_impl_from_primitive!(i8);
number_impl_from_primitive!(i16);
number_impl_from_primitive!(i32);
number_impl_from_primitive!(i64);
number_impl_from_primitive!(i128);

number_impl_from_primitive!(u8);
number_impl_from_primitive!(u16);
number_impl_from_primitive!(u32);
number_impl_from_primitive!(u64);
number_impl_from_primitive!(u128);

number_impl_from_primitive!(f32);
number_impl_from_primitive!(f64);

number_impl_from_primitive!(usize);
number_impl_from_primitive!(isize);

#[test]
fn test_pretty_print() {
    assert_eq!(Number::from_f64(1.).pretty_print(), "1");
    assert_eq!(Number::from_f64(100.).pretty_print(), "100");
    assert_eq!(Number::from_f64(1.234).pretty_print(), "1.234");
    assert_eq!(Number::from_f64(12345.6).pretty_print(), "12345.6");
    assert_eq!(Number::from_f64(1.234e50).pretty_print(), "1.234e+50");
    assert_eq!(Number::from_f64(-1.234e50).pretty_print(), "-1.234e+50");
    assert_eq!(Number::from_f64(1.234e-50).pretty_print(), "1.234e-50");
    assert_eq!(Number::from_f64(-1.234e-50).pretty_print(), "-1.234e-50");

    assert_eq!(Number::from_f64(1234.).pretty_print(), "1234");
    assert_eq!(Number::from_f64(12345.).pretty_print(), "12345");
    assert_eq!(Number::from_f64(123456.).pretty_print(), "123_456");
    assert_eq!(
        Number::from_f64(1234567890.).pretty_print(),
        "1_234_567_890"
    );
    assert_eq!(
        Number::from_f64(1234567890000000.).pretty_print(),
        "1.23457e+15"
    );

    assert_eq!(Number::from_f64(1.23456789).pretty_print(), "1.23457");
    assert_eq!(
        Number::from_f64(1234567890000.1).pretty_print(),
        "1.23457e+12"
    );

    assert_eq!(Number::from_f64(100.00001).pretty_print(), "100.0");

    assert_eq!(Number::from_f64(0.00001).pretty_print(), "0.00001");
    assert_eq!(Number::from_f64(0.000001).pretty_print(), "0.000001");
    assert_eq!(Number::from_f64(0.0000001).pretty_print(), "1.0e-7");
}

#[test]
fn test_abs() {
    assert_eq!(Number::from_f64(0.0).abs(), Number::from_f64(0.0));
    assert_eq!(Number::from_f64(1.0).abs(), Number::from_f64(1.0));
    assert_eq!(Number::from_f64(-1.0).abs(), Number::from_f64(1.0));
}
