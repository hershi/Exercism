extern crate spreadsheet_numbering;
use spreadsheet_numbering::*;

#[test]
fn test_single_digit() {
    assert_eq!("a", get_column_name(1, 26));
    assert_eq!("b", get_column_name(2, 26));
    assert_eq!("c", get_column_name(3, 26));
}

#[test]
fn test_double_digit() {
    assert_eq!("ga", get_column_name(183, 26));
    assert_eq!("xt", get_column_name(644, 26));
    assert_eq!("cc", get_column_name(81, 26));
}

#[test]
fn triple_digit() {
    assert_eq!("xfd", get_column_name(16384, 26));
}

#[test]
fn test_single_digit_2() {
    assert_eq!("a", get_column_name_2(1, 26));
    assert_eq!("b", get_column_name_2(2, 26));
    assert_eq!("c", get_column_name_2(3, 26));
}

#[test]
fn test_double_digit_2() {
    assert_eq!("ga", get_column_name_2(183, 26));
    assert_eq!("xt", get_column_name_2(644, 26));
    assert_eq!("cc", get_column_name_2(81, 26));
}

#[test]
fn triple_digit_2() {
    assert_eq!("xfd", get_column_name_2(16384, 26));
}