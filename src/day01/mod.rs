pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn calibration_values_sum(input: &str) -> u32 {
    input.split('\n').map(calibration_value).sum()
}

pub fn calibration_value(input: &str) -> u32 {
    let mut first = None;
    let mut last = None;

    for i in 0..input.len() {
        if let Some(value) = to_value(&input[i..]) {
            if first.is_none() {
                first = Some(value);
            }
            last = Some(value);
        }
    }
    10 * first.unwrap_or(0) + last.unwrap_or(0)
}

const MATCHES: [(&str, u32); 19] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn to_value(input: &str) -> Option<u32> {
    for (pattern, value) in MATCHES {
        if input.starts_with(pattern) {
            return Some(value);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calibration_value1() {
        let line = "1abc2";
        assert_eq!(calibration_value(line), 12);
    }
    #[test]
    fn calibration_value2() {
        let line = "pqr3stu8vwx";
        assert_eq!(calibration_value(line), 38);
    }
    #[test]
    fn calibration_value3() {
        let line = "a1b2c3d4e5f";
        assert_eq!(calibration_value(line), 15);
    }
    #[test]
    fn calibration_value4() {
        let line = "treb7uchet";
        assert_eq!(calibration_value(line), 77);
    }
    #[test]
    fn sum_of_calabration_values() {
        let input = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "#;

        assert_eq!(calibration_values_sum(input), 142);
    }
    #[test]
    fn sum_of_calabration_values_part_2() {
        let input = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#;

        assert_eq!(calibration_values_sum(input), 281);
    }
}
