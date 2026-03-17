use bigdecimal::BigDecimal;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

const PI_DIGITS: &str = include_str!("pi-digits.txt"); // Thanks to https://www.exeter.ac.uk/v8media/facultysites/ese/physics/pi6.txt (with whitespace removed)

#[wasm_bindgen]
pub fn score_integer(integer_string: &str) -> String {
    let decimal_input_result = BigDecimal::from_str(integer_string.trim());

    match decimal_input_result {
        Err(e) => format!("There's a problem with the input. Cause: {}.", e),
        Ok(v) if v != bigdecimal_floor(&v) => {
            "The input is not an integer. It has digits after the decimal point.".to_string()
        }
        Ok(v) if v < 0 => "The input is negative. It must be a non-negative integer.".to_string(),
        Ok(v) => match matching_decimal_places_in_root(&v) {
            Some(v) => format!(
                "The first {} digits after the decimal point of the square root match the digits of pi.",
                v
            ),
            None => format!(
                "Impressive. As many digits match as I have digits of pi stored, so your score is at least {}.",
                PI_DIGITS
                    .split_whitespace()
                    .nth(0)
                    .expect("PI_DIGITS has at least 1 line.")
                    .len()
            ),
        },
    }
}

fn matching_decimal_places_in_root(value: &BigDecimal) -> Option<usize> {
    let mut approximate_square_root =
        BigDecimal::from_str("1").expect("1 is a number that can be parsed.");
    let mut number_of_matches: usize = 0;

    loop {
        loop {
            if root_inaccurate_at(number_of_matches, &value, &approximate_square_root) {
                approximate_square_root = improved_guess(&value, &approximate_square_root)
            } else {
                break;
            }
        }

        match pi_mismatch_at(number_of_matches, &approximate_square_root) {
            Some(true) => break,
            Some(false) => (),
            None => return None,
        }

        number_of_matches += 1;
    }

    Some(number_of_matches)
}

fn pi_mismatch_at(index: usize, root: &BigDecimal) -> Option<bool> {
    let pi_digit = PI_DIGITS
        .split_whitespace()
        .nth(0)
        .expect("PI_DIGITS has at least 1 line.")
        .chars()
        .nth(index)?;
    let root_string = root.to_string();
    let components = root_string.split(".").collect::<Vec<&str>>();
    let root_digit_option = if components.len() == 1 {
        None
    } else {
        components[1].chars().nth(index)
    };
    match root_digit_option {
        Some(digit) => Some(digit != pi_digit),
        None => Some(true),
    }
}

fn root_inaccurate_at(index: usize, value: &BigDecimal, root: &BigDecimal) -> bool {
    let truncated_root = truncate_to(index + 1, root);
    squared(&truncated_root) > *value || squared(&incremented_at(index, &truncated_root)) <= *value
}

fn incremented_at(index: usize, value: &BigDecimal) -> BigDecimal {
    value
        + BigDecimal::from_str(&format!("0.{}1", "0".repeat(index)))
            .expect("A string of only digits can be parsed.")
}

fn improved_guess(value: &BigDecimal, root: &BigDecimal) -> BigDecimal {
    (root + value / root) / 2
}

fn truncate_to(index: usize, root: &BigDecimal) -> BigDecimal {
    let scalar = &BigDecimal::from_str(&format!("1{}", "0".repeat(index)))
        .expect("A string of only digits can be parsed.");
    bigdecimal_floor(&(root * scalar)) / scalar
}

fn bigdecimal_floor(value: &BigDecimal) -> BigDecimal {
    BigDecimal::from_str(value.to_string().split(".").collect::<Vec<&str>>()[0])
        .expect("Can form a BigDecimal from the integer part of another.")
}

fn squared(value: &BigDecimal) -> BigDecimal {
    value * value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_integer_with_empty_input_fails() {
        let text = "";
        let result = score_integer(text);
        assert_eq!(
            result,
            "There's a problem with the input. Cause: Failed to parse empty string."
        );
    }

    #[test]
    fn score_integer_with_letter_input_fails() {
        let text = "a";
        let result = score_integer(text);
        assert_eq!(
            result,
            "There's a problem with the input. Cause: invalid digit found in string."
        );
    }

    #[test]
    fn score_integer_with_non_integer_input_fails() {
        let text = "1.1";
        let result = score_integer(text);
        assert_eq!(
            result,
            "The input is not an integer. It has digits after the decimal point."
        );
    }

    #[test]
    fn score_integer_with_eleven_input_gives_two() {
        let text = "11";
        let result = score_integer(text);
        assert_eq!(result, "The first 2 digits after the decimal point of the square root match the digits of pi.");
    }

    #[test]
    fn score_integer_with_128_input_gives_two() {
        let text = "128";
        let result = score_integer(text);
        assert_eq!(result, "The first 2 digits after the decimal point of the square root match the digits of pi.");
    }

    #[test]
    fn score_integer_with_5853081_input_gives_two() {
        let text = "5853081";
        let result = score_integer(text);
        assert_eq!(result, "The first 6 digits after the decimal point of the square root match the digits of pi.");
    }

    #[test]
    fn matching_decimal_places_in_root_with_eleven_input_gives_two() {
        let value = BigDecimal::from_str("11").expect("11 is valid.");
        let result = matching_decimal_places_in_root(&value)
            .expect("There are sufficient pi digits for 11.");
        assert_eq!(result, 2);
    }

    #[test]
    fn root_inaccurate_at_with_square_number_is_correct_at_zero() {
        let value = BigDecimal::from_str("4").expect("Digits and decimal point are valid.");
        let root = BigDecimal::from_str("2").expect("Digits and decimal point are valid.");
        let result = root_inaccurate_at(0, &value, &root);
        assert_eq!(result, false);
    }

    #[test]
    fn root_inaccurate_at_with_square_number_is_correct_at_one() {
        let value = BigDecimal::from_str("4").expect("Digits and decimal point are valid.");
        let root = BigDecimal::from_str("2").expect("Digits and decimal point are valid.");
        let result = root_inaccurate_at(1, &value, &root);
        assert_eq!(result, false);
    }

    #[test]
    fn root_inaccurate_at_with_two_is_correctly_false_at_zero() {
        let value = BigDecimal::from_str("2").expect("Digits and decimal point are valid.");
        let root = BigDecimal::from_str("1.41").expect("Digits and decimal point are valid.");
        let result = root_inaccurate_at(0, &value, &root);
        assert_eq!(result, false);
    }

    #[test]
    fn root_inaccurate_at_with_two_is_correctly_false_at_one() {
        let value = BigDecimal::from_str("2").expect("Digits and decimal point are valid.");
        let root = BigDecimal::from_str("1.41").expect("Digits and decimal point are valid.");
        let result = root_inaccurate_at(1, &value, &root);
        assert_eq!(result, false);
    }

    #[test]
    fn root_inaccurate_at_with_two_is_correctly_true_at_two() {
        let value = BigDecimal::from_str("2").expect("Digits and decimal point are valid.");
        let root = BigDecimal::from_str("1.4").expect("Digits and decimal point are valid.");
        let result = root_inaccurate_at(2, &value, &root);
        assert_eq!(result, true);
    }

    #[test]
    fn root_inaccurate_at_with_four_is_correctly_true_at_two() {
        let value = BigDecimal::from_str("4").expect("Digits and decimal point are valid.");
        let root = BigDecimal::from_str("1.99").expect("Digits and decimal point are valid.");
        let result = root_inaccurate_at(2, &value, &root);
        assert_eq!(result, true);
    }

    #[test]
    fn root_inaccurate_at_with_one_is_correctly_false_at_zero() {
        let value = BigDecimal::from_str("1").expect("1 is valid.");
        let root = BigDecimal::from_str("1").expect("1 is valid.");
        let result = root_inaccurate_at(0, &value, &root);
        assert_eq!(result, false);
    }

    #[test]
    fn root_inaccurate_at_with_one_is_correctly_false_at_one() {
        let value = BigDecimal::from_str("1").expect("1 is valid.");
        let root = BigDecimal::from_str("1").expect("1 is valid.");
        let result = root_inaccurate_at(1, &value, &root);
        assert_eq!(result, false);
    }

    #[test]
    fn root_inaccurate_at_with_one_is_correctly_false_at_two() {
        let value = BigDecimal::from_str("1").expect("1 is valid.");
        let root = BigDecimal::from_str("1").expect("1 is valid.");
        let result = root_inaccurate_at(2, &value, &root);
        assert_eq!(result, false);
    }

    #[test]
    fn pi_mismatch_at_with_one_at_zero_is_correctly_true() {
        let index = 0;
        let value = BigDecimal::from_str("1").expect("1 is valid.");
        let result = pi_mismatch_at(index, &value);
        assert_eq!(result, Some(true));
    }

    #[test]
    fn truncate_to_zero_is_correct() {
        let value = BigDecimal::from_str("5.4321").expect("Digits and decimal point are valid.");
        let trunk = truncate_to(0, &value);
        assert_eq!(
            trunk,
            BigDecimal::from_str("5").expect("5 is a valid digit.")
        );
    }

    #[test]
    fn truncate_to_one_is_correct() {
        let value = BigDecimal::from_str("5.4321").expect("Digits and decimal point are valid.");
        let trunk = truncate_to(1, &value);
        assert_eq!(
            trunk,
            BigDecimal::from_str("5.4").expect("Digits and decimal point are valid.")
        );
    }

    #[test]
    fn truncate_to_more_than_present_is_correct() {
        let value = BigDecimal::from_str("5.4321").expect("Digits and decimal point are valid.");
        let trunk = truncate_to(7, &value);
        assert_eq!(
            trunk,
            BigDecimal::from_str("5.4321").expect("Digits and decimal point are valid.")
        );
    }

    #[test]
    fn bigdecimal_floor_is_correct_for_integer() {
        let value = BigDecimal::from_str("5").expect("5 is a valid digit.");
        let floor = bigdecimal_floor(&value);
        assert_eq!(
            floor,
            BigDecimal::from_str("5").expect("5 is a valid digit.")
        );
    }

    #[test]
    fn bigdecimal_floor_is_correct_for_non_integer() {
        let value = BigDecimal::from_str("5.4321").expect("Digits and decimal point are valid.");
        let floor = bigdecimal_floor(&value);
        assert_eq!(
            floor,
            BigDecimal::from_str("5").expect("5 is a valid digit.")
        );
    }

    #[test]
    fn squared_zero_is_zero() {
        let zero = BigDecimal::from_str("0").expect("0 is a valid digit.");
        let zero_squared = squared(&zero);
        assert_eq!(zero, zero_squared);
    }

    #[test]
    fn squared_decimal_is_correct() {
        let value = BigDecimal::from_str("5.4321").expect("Digits and decimal point are valid.");
        let value_squared = squared(&value);
        assert_eq!(
            value_squared,
            BigDecimal::from_str("29.50771041").expect("Digits and decimal point are valid.")
        );
    }
}
