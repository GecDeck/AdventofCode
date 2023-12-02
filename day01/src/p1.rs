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
    for char in input.chars() {
        if char.is_numeric() {
            let number: u32 = char.to_digit(10)
                .expect("converting first numeric char to base 10 number");
            return number * 10;
            // All numbers are 2 digits so the first number is multiplied by ten
            //  so it will be in the tens column
        }
    }
    panic!("Can't get first number from an empty string");
}

fn get_last_num(input: &str) -> u32 {
    for char in input.chars().rev() {
        if char.is_numeric() {
            let number: u32 = char.to_digit(10)
                .expect("converting last numeric char to base 10 number");
            return number;
        }
    }

    panic!("Can't get last number from an empty string");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input: &str =
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        assert_eq!(142, process(input));
    }

    #[test]
    fn test_first_number() {
        let input: &str = "one2";
        assert_eq!(20, get_first_num(input));
    }

    #[test]
    fn test_last_number() {
        let input: &str = "one2one4";
        assert_eq!(4, get_last_num(input));

        let input_one_number: &str = "one2one";
        assert_eq!(2, get_last_num(input_one_number));
    }
}
