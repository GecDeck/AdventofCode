const NUMBERS: [(&str, u32); 9] =
[("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

pub fn process(input: &str) -> u32 {
    let mut numbers: Vec<u32> = vec![];
    for line in input.lines() {
        // get first number
        // get last number
        // format string with {first}{last}
        // parse into u32
        let first_digit: u32 = get_first_num(line);
        let last_digit: u32 = get_last_num(line);
        let number: u32 = first_digit + last_digit;
        numbers.push(number);
    }

    let sum: u32 = numbers.iter().map(|number| number).sum();
    return sum;
}

fn get_first_num(input: &str) -> u32 {
    // Checks for a number then returns it
    //  or removes first character if it doesn't and checks again
    for index in 0..=input.len() {
        let modified_string: &str = &input[index..input.len()];
        for number in NUMBERS {
            if modified_string.starts_with(number.0) {
                return number.1 * 10;
            }
        }

        let first_char: char = modified_string.chars().next().expect("getting first char from input");
        if first_char.is_numeric() {
            let number: u32 = first_char.to_digit(10)
                .expect("converting last numeric char to base 10 number");
            return number * 10;
        }
    }

    panic!("Can't get first number");
}

fn get_last_num(input: &str) -> u32 {
    // Checks for a number then returns it
    //  or removes last character if it doesn't and checks again
    for index in 0..=input.len() {
        let modified_string: &str = &input[0..input.len() - index];
        for number in NUMBERS {
            if modified_string.ends_with(number.0) {
                return number.1;
            }
        }

        let last_char: char = modified_string.chars().last().expect("getting last character from input");
        if last_char.is_numeric() {
            let number: u32 = last_char.to_digit(10)
                .expect("converting last numeric char to base 10 number");
            return number;
        }
    }

    panic!("Can't get last number");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input: &str =
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        assert_eq!(281, process(input));
    }

    #[test]
    fn test_first_number() {
        let input: &str = "one2";
        assert_eq!(10, get_first_num(input));
    }

    #[test]
    fn test_last_number() {
        let input: &str = "one2one4eight";
        assert_eq!(8, get_last_num(input));

        let input_one_number: &str = "2onekokokok";
        assert_eq!(1, get_last_num(input_one_number));

        let input_one_char: &str = "1";
        assert_eq!(1, get_last_num(input_one_char));
    }
}
