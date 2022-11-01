use pretty_bytes_rust::pretty_bytes;
use pretty_bytes_rust::PrettyBytesOptions;

#[test]
fn pretty_bytes_no_option1() {
    let result = pretty_bytes(1023, None);
    assert_eq!(result, "1023.00 B");
}

#[test]
fn pretty_bytes_no_option2() {
    let result = pretty_bytes(1024, None);
    assert_eq!(result, "1.00 kB");
}

#[test]
fn pretty_bytes_option1() {
    let result = pretty_bytes(
        1024 * 1024 * 3,
        Some(PrettyBytesOptions {
            use_1024_instead_of_1000: None,
            number_of_decimal: Some(3),
            remove_zero_decimal: None,
        }),
    );
    assert_eq!(result, "3.000 MB");
}

#[test]
fn pretty_bytes_option2() {
    let result = pretty_bytes(
        1024 * 1024 * 3,
        Some(PrettyBytesOptions {
            use_1024_instead_of_1000: None,
            number_of_decimal: Some(3),
            remove_zero_decimal: Some(true),
        }),
    );
    assert_eq!(result, "3 MB");
}

#[test]
fn pretty_bytes_option3() {
    let result = pretty_bytes(
        1024 * 1024 * 3,
        Some(PrettyBytesOptions {
            use_1024_instead_of_1000: Some(false),
            number_of_decimal: Some(3),
            remove_zero_decimal: Some(false),
        }),
    );
    assert_eq!(result, "3.146 MB");
}

#[test]
fn pretty_bytes_option4() {
    let r2 = pretty_bytes(
        1024 * 1024 * 8 + 123,
        Some(PrettyBytesOptions {
            use_1024_instead_of_1000: Some(false),
            number_of_decimal: None,
            remove_zero_decimal: Some(false),
        }),
    );
    assert_eq!(r2, "8.39 MB");
}
