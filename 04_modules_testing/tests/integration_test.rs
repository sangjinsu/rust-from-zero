// tests/integration_test.rs
use modules04::calculator;
use modules04::utils;
#[test]
fn test_add_integration() {
    assert_eq!(calculator::add(3, 4), 7);
}

#[test]
fn test_subtract_integration() {
    assert_eq!(calculator::subtract(10, 5), 5);
}

#[test]
fn test_multiply_integration() {
    assert_eq!(calculator::multiply(3, 4), 12);
}

#[test]
fn test_divide_integration() {
    assert_eq!(calculator::divide(10, 2), 5);
}

#[test]
fn test_divide_by_zero_integration() {
    let result = std::panic::catch_unwind(|| {
        calculator::divide(10, 0);
    });
    assert!(result.is_err());
}

#[test]
fn test_reverse_string_integration() {
    assert_eq!(utils::reverse_string("hello"), "olleh");
}
