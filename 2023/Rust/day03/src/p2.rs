use std::collections::HashMap;

pub fn process(input: &str) -> u64 {
    // A valid part number is one where it touches a symbol even diagonally
    // find symbol indices
    // check line, line before and line after at indices +- 1 of symbol
    // if you find a number walk forward and backward until you hit a non numeric value
    // get numbers and add to sum
    let symbols: Vec<Symbol> = find_symbols(input);
    let positions: Vec<Position> = find_number_positions(input, &symbols);
    let numbers: Vec<u64> = find_numbers(input, &positions);

    let sum: u64 = numbers.iter().map(|x| x).sum();
    return sum;
}

#[derive(Clone, Copy, PartialEq)]
struct Symbol {
    index: usize,
    line: usize,
    char: char,
}
impl Symbol {
    fn from(index: usize, line: usize, char: char) -> Self {
        Self {
            index,
            line,
            char,
        }
    }

    fn is_gear(&self) -> bool {
        if self.char == '*' {
            return true;
        }
        return false;
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Position {
    index: usize,
    line: usize,
    gear: bool,
    symbol: Symbol,
}
impl Position {
    fn from(index: usize, line: usize, gear: bool, symbol: Symbol) -> Self {
        Self {
            index,
            line,
            gear,
            symbol,
        }
    }

    fn is_same_position(&self, index: usize, line: usize) -> bool {
        if self.index == index && self.line == line {
            return true;
        }
        return false;
    }
}

struct Number {
    value: u64,
    position: Position,
}
impl Number {
    fn from(value: u64, position: Position) -> Self {
        Self {
            value,
            position,
        }
    }
}

fn find_symbols(input: &str) -> Vec<Symbol> {
    // Given a string return a vec of all symbols where (index in line, line number)

    let mut symbols: Vec<Symbol> = vec![];
    for (line_num, line) in input.lines().enumerate() {
        for (index, char) in line.trim().chars().enumerate() {
            if !char.is_digit(10) && char != '.' {
                symbols.push(Symbol::from(index, line_num, char));
            }
        }
    }

    return symbols;
}

fn find_number_positions(input: &str, symbols: &[Symbol]) -> Vec<Position> {
    // Given a string and list of symbols return a vector of numbers bordering those symbols

    let mut adjacent_number_indices: Vec<Position> = vec![];
    // index in line, line number

    let lines: Vec<&str> = input.lines().collect();
    for symbol in symbols {

        let lower_range = if symbol.line == 0 { 0 } else { symbol.line - 1 };
        let upper_range = if symbol.line == lines.len() - 1 { lines.len() - 1 } else { symbol.line + 1 };

        for (relative_line, line) in lines[lower_range..=upper_range].iter().enumerate() {
            let chars: Vec<char> = line.trim().chars().collect();

            let lower_range = if symbol.index == 0 { 0 } else { symbol.index - 1 };
            let upper_range = if symbol.index == chars.len() - 1 { chars.len() - 1 } else { symbol.index + 1 };

            for (relative_index, char) in chars[lower_range..=upper_range].iter().enumerate() {
                if char.is_digit(10) {
                    let line_number: usize = match relative_line {
                        0 => symbol.line - 1,
                        1 => if symbol.line == 0 { symbol.line + 1 } else { symbol.line },
                        2 => symbol.line + 1,
                        _ => panic!(),
                    };

                    let index: usize = match relative_index {
                        0 => if symbol.index == 0 { 0 } else { symbol.index - 1 },
                        1 => symbol.index,
                        2 => if symbol.index == chars.len() - 1 { chars.len() - 1 } else { symbol.index + 1 },
                        _ => panic!(),
                    };

                    adjacent_number_indices.push(Position::from(index, line_number, symbol.is_gear(), *symbol));
                }
            }
        }
    }

    return adjacent_number_indices;
}

fn find_numbers(input: &str, number_positions: &[Position]) -> Vec<u64> {
    // walk through line gathering numbers and the index of each digit
    // check each number to check if any of its digits are found in number_positions

    let mut number_map: HashMap<(usize, usize), Number> = HashMap::new();
    for (line_num, line) in input.lines().enumerate() {
        let mut number: Vec<char> = vec![];
        let mut digit_positions: Vec<(usize, usize)> = vec![];
        for (index, char) in line.trim().chars().enumerate() {
            if char.is_digit(10) {
                number.push(char);
                digit_positions.push((index, line_num));
            }

            if !char.is_digit(10) || index == line.trim().len() - 1 {
                for digit_position in &digit_positions {
                    for position in number_positions {
                        if position.is_same_position(digit_position.0, digit_position.1) && number.len() != 0 {
                            let number_string: String = number.iter().collect();
                            let number_u64: u64 = number_string.parse().expect("parsing string into u64");

                            match number_map.get(digit_position) {
                                Some(_) => { },
                                None => { number_map.insert(*digit_position, Number::from(number_u64, *position)); },
                            }
                            number.clear();
                        }
                    }
                }
                number.clear();
                digit_positions.clear();
            }
        }
    }

    let mut gear_groups: Vec<(u64, u64)> = vec![];
    for (_, v) in &number_map {
        if v.position.gear {
            for (_, second_number) in &number_map {
                if second_number.position.symbol == v.position.symbol &&
                    !gear_groups.contains(&(second_number.value, v.value)) &&
                    v.position != second_number.position {
                    gear_groups.push((v.value, second_number.value));
                    println!("{:?}", gear_groups);
                }
            }
        }
    }

    let mut numbers: Vec<u64> = vec![];
    for group in gear_groups {
        numbers.push(group.0 * group.1);
    }
    return numbers;
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
        "467..114..
         ...*......
         ..35..633.
         ......#...
         617*......
         .....+.58.
         ..592.....
         ......755.
         ...$.*....
         .664.598..";

    #[test]
    fn test_process() {
        assert_eq!(467835, process(INPUT));
    }

    #[test]
    fn test_find_symbols() {
    }

    #[test]
    fn test_find_number_positions() {
    }

    #[test]
    fn test_find_numbers() {
    }
}
