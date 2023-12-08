use std::collections::HashMap;
use itertools::Itertools;

const START: &str = "AAA";
const END: &str = "ZZZ";
const RIGHT: char = 'R';
const LEFT: char = 'L';

pub fn process(input: &str) -> u32 {
    let directions: &str = get_directions(input);
    let map: HashMap<&str, Path> = get_map(input);
    let steps: u32 = navigate_map(directions, map);
    return steps;
}

fn get_directions(input: &str) -> &str {
    return input
        .lines().next().expect("getting first line of input with directions")
        .trim();
}

struct Path {
    left: String,
    right: String,
}
impl Path {
    fn new(input: &str) -> Self {
        // receives (BBB, CCC)
        let input: String = input
            .replace('(', "")
            .replace(')', "")
            .replace(',', "");
        // BBB CCC
        let (left, right): (&str, &str) = input.split_whitespace()
                             .collect_tuple().expect("collecting directions into tuple");
        Self {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

fn get_map(input: &str) -> HashMap<&str, Path> {
    let mut map: HashMap<&str, Path> = HashMap::new();

    for line in input.lines() {
        if line.contains('=') {
            let location: &str = line
                .trim()
                .split('=')
                .next().expect("getting location portion of line")
                .trim();
            let directions: &str = line
                .trim()
                .split('=')
                .last().expect("getting directions portion of line");
            let path: Path = Path::new(directions);
            map.insert(location, path);
        }
    }

    return map;
}

fn navigate_map(directions: &str, map: HashMap<&str, Path>) -> u32 {
    let mut steps: u32 = 0;
    let at_zzz: bool = false;

    let mut current_location: &str = START;
    while !at_zzz {
        for char in directions.chars() {
            let paths: &Path = map.get(current_location)
                .expect("getting directions from current location");
            match char {
                LEFT => current_location = &paths.left,
                RIGHT => current_location = &paths.right,
                _ => panic!("unhandled char found"),
            }
            steps += 1;
            if current_location == END {
                return steps;
            }
        }
    }

    println!("{}", steps);
    return steps;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str =
        "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_process() {
        assert_eq!(6, process(INPUT));
    }

    #[test]
    fn test_directions() {
        assert_eq!("LLR", get_directions(INPUT));
    }
}
