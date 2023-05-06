//! pretty_bytes takes a [u64] that represent a number of bytes and output a [String] in prettier way that make
//! sense for a human being.
//!
//! Provide customizations to use calculation with `1000` or `1024` as well as how many decimal to display.
//!
mod pretty_bytes_lib;
// Expose
pub use crate::pretty_bytes_lib::PrettyBytesOptions;
// Do not expose
use crate::pretty_bytes_lib::{PrettyBytesOptionWithDefault, BIT_UNITS, BYTE_UNITS};
use std::cmp;

/// pretty_bytes main function that convert bytes into a pretty format that is easier
/// to read for a humain
///
/// # Arguments
/// The first argument is the number of bytes to transform
///
/// The second argument is the option. Can be set to `None` to use the default of 2 decimals and 1024 bytes per unit
///  
/// # Examples
/// ```rust
/// use pretty_bytes_rust::pretty_bytes;
/// let r1 = pretty_bytes(1024 * 1024 * 5 + 50000, None);
/// assert_eq!(r1, "5.05 MB");
///
/// ```
///
/// ```rust
/// use pretty_bytes_rust::pretty_bytes;
/// use pretty_bytes_rust::PrettyBytesOptions;
/// let r2 = pretty_bytes(1024 * 1024 * 9 + 123, Some(PrettyBytesOptions {
///           use_1024_instead_of_1000: Some(false),
///           number_of_decimal: Some(3),
///           remove_zero_decimal: Some(false)
///        }));
/// assert_eq!(r2, "9.437 MB");
/// ```
pub fn pretty_bytes(bytes: u64, options: Option<PrettyBytesOptions>) -> String {
    let options_with_default = set_default_options(options);

    let delimiter = if options_with_default.use_1024_instead_of_1000 {
        1024_f64
    } else {
        1000_f64
    };

    let units = if options_with_default.use_1024_instead_of_1000 {
        BIT_UNITS
    } else {
        BYTE_UNITS
    };

    let max_units_index = units.len() - 1;
    let index = get_unit_index(bytes as f64, delimiter, max_units_index as i32);
    let pretty_bytes = get_string(
        bytes as f64,
        delimiter,
        index,
        options_with_default.number_of_decimal,
        options_with_default.remove_zero_decimal,
    );
    let unit = BYTE_UNITS[index as usize];
    format!("{} {}", pretty_bytes, unit)
}

fn get_unit_index(bytes: f64, delimiter: f64, max_units_index: i32) -> i32 {
    println!("Bytes {}", bytes as f64);
    println!("Delimiter {}", delimiter);
    println!("Bytes log {}", (bytes as f64).ln());
    println!("Delimeter log {}", delimiter.ln());
    let mut bytes_log = (bytes as f64).ln();
    if bytes_log < 0.0 {
        bytes_log = 0.0;
    }
    cmp::min(
        (bytes_log / delimiter.ln()).floor() as i32,
        max_units_index as i32,
    )
}

fn get_string(
    bytes: f64,
    delimiter: f64,
    index: i32,
    number_of_decimal: usize,
    remove_zero_decimal: bool,
) -> String {
    let result = bytes as f64 / delimiter.powi(index);
    let mut number_of_decimal_applied = number_of_decimal;
    if remove_zero_decimal {
        let result_without_decimal = result as i64 as f64;
        if result_without_decimal == result {
            number_of_decimal_applied = 0;
        }
    }
    format!("{:.1$}", result, number_of_decimal_applied)
}

fn set_default_options(user_options: Option<PrettyBytesOptions>) -> PrettyBytesOptionWithDefault {
    // Ensure if we the user passed nothing that we have a type with all options with no value
    let output_format = user_options.unwrap_or(PrettyBytesOptions {
        use_1024_instead_of_1000: None,
        number_of_decimal: None,
        remove_zero_decimal: None,
    });

    // Give default value to all options not defined by the user
    PrettyBytesOptionWithDefault {
        use_1024_instead_of_1000: output_format.use_1024_instead_of_1000.unwrap_or(true),
        number_of_decimal: output_format.number_of_decimal.unwrap_or(2),
        remove_zero_decimal: output_format.remove_zero_decimal.unwrap_or(false),
    }
}
#[cfg(test)]
mod test_set_default_options {
    use super::*;

    #[test]
    fn test_set_default_options_no_option() {
        let result = set_default_options(None);
        assert_eq!(result.number_of_decimal, 2, "Number of decimal");
        assert_eq!(result.use_1024_instead_of_1000, true, "Default 1024");
        assert_eq!(result.remove_zero_decimal, false, "Remove zero decimal");
    }
    #[test]
    fn test_set_default_options_use_user_option() {
        let result = set_default_options(Some(PrettyBytesOptions {
            number_of_decimal: Some(5),
            use_1024_instead_of_1000: Some(false),
            remove_zero_decimal: Some(false),
        }));
        assert_eq!(result.number_of_decimal, 5, "Number of decimal");
        assert_eq!(result.use_1024_instead_of_1000, false, "Default 1024");
        assert_eq!(result.remove_zero_decimal, false, "Remove zero decimal");
    }
}
#[cfg(test)]
mod test_get_unit_index {
    use super::*;
    #[test]
    fn test_get_unit_0() {
        let result = get_unit_index(0_f64, 1024_f64, 9);
        assert_eq!(result, 0)
    }
    #[test]
    fn test_get_unit_1024_index_1023() {
        let result = get_unit_index(1023_f64, 1024_f64, 9);
        assert_eq!(result, 0)
    }
    #[test]
    fn test_get_unit_1024_index_1024() {
        let result = get_unit_index(1024_f64, 1024_f64, 9);
        assert_eq!(result, 1)
    }
    #[test]
    fn test_get_unit_1000_index_999() {
        let result = get_unit_index(999_f64, 1000_f64, 9);
        assert_eq!(result, 0)
    }
    #[test]
    fn test_get_unit_1000_index_1000() {
        let result = get_unit_index(1000_f64, 1000_f64, 9);
        assert_eq!(result, 1)
    }
    #[test]
    fn test_get_unit_1024_index_1mb() {
        let result = get_unit_index(1024_f64 * 1024_f64 * 1024_f64, 1024_f64, 9);
        assert_eq!(result, 3)
    }
}

#[cfg(test)]
mod test_get_string {
    use super::*;

    #[test]
    fn test_get_string_bytes() {
        let result = get_string(1023_f64, 1024_f64, 0, 2, false);
        assert_eq!(result, "1023.00")
    }

    #[test]
    fn test_get_string_bytes_remove_decimal() {
        let result = get_string(1023_f64, 1024_f64, 0, 2, true);
        assert_eq!(result, "1023")
    }

    #[test]
    fn test_get_string_kilobytes() {
        let result = get_string(1024_f64, 1024_f64, 1, 2, false);
        assert_eq!(result, "1.00")
    }
    #[test]
    fn test_get_string_kilobytes_remove_decimal() {
        let result = get_string(1024_f64, 1024_f64, 1, 2, true);
        assert_eq!(result, "1")
    }

    #[test]
    fn test_get_string_kilobytes_no_even() {
        let result = get_string(1100_f64, 1024_f64, 1, 2, false);
        assert_eq!(result, "1.07")
    }

    #[test]
    fn test_get_string_kilobytes_no_even_remove_decimal() {
        let result = get_string(1100_f64, 1024_f64, 1, 2, true);
        assert_eq!(result, "1.07")
    }
    #[test]
    fn test_get_string_kilobytes_no_even_more_decimal() {
        let result = get_string(1100_f64, 1024_f64, 1, 3, false);
        assert_eq!(result, "1.074")
    }
    #[test]
    fn test_get_string_kilobytes_no_even_more_decimal_remove_decimal() {
        let result = get_string(1100_f64, 1024_f64, 1, 3, true);
        assert_eq!(result, "1.074")
    }
}
