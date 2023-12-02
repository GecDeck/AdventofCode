pub fn process(input: &str) -> u32 {
    let mut powers: Vec<u32> = vec![];
    for line in input.lines() {
        let game: Game = Game::from_str(line);
        let power: u32 = game.min_colours.0 * game.min_colours.1 * game.min_colours.2;
        powers.push(power);
    }

    let sum_of_power: u32 = powers.iter().map(|power| power).sum();
    return  sum_of_power;
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    pulls: Vec<(u32, u32, u32)>,
    // (red, green, blue)
    min_colours: (u32, u32, u32),
}
impl Game {
    fn from_str(input: &str) -> Self {
        let id: u32 = input.trim().split_at(4).1
            .split(":").next().expect("getting id of input")
            .trim().parse().expect("parsing id");

        let pulls: Vec<(u32, u32, u32)> = input.split(":").last()
            .expect("getting second half of input")
            .split(";")
            .map( |pull|
                  parse_pull(pull.split(",").collect())
                )
            .collect();

        let min_colours: (u32, u32, u32) = get_mins(&pulls);

        Self {
            id,
            pulls,
            min_colours,
        }
    }

}

fn parse_pull(pull: Vec<&str>) -> (u32, u32, u32) {
    // Parses string slices and returns a tuple containing the number of balls of each colour
    let mut pull_tuple: (u32, u32, u32) = (0, 0, 0);
    for colour in pull {
        if colour.contains("red") {
            let red: u32 = colour.replace("red", "").trim().parse()
                .expect("parsing red into number");
            pull_tuple.0 = red;
        }
        if colour.contains("green") {
            let green: u32 = colour.replace("green", "").trim().parse()
                .expect("parsing green into number");
            pull_tuple.1 = green;
        }
        if colour.contains("blue") {
            let blue: u32 = colour.replace("blue", "").trim().parse()
                .expect("parsing blue into number");
            pull_tuple.2 = blue;
        }
    }
    return pull_tuple;
}

fn get_mins(pulls: &[(u32, u32, u32)]) -> (u32, u32, u32) {
    // Returns minimun number of cubes of each colour needed
    let mut mins: (u32, u32, u32) = (0, 0, 0);
    for pull in pulls {
        if pull.0 > mins.0 {
            mins.0 = pull.0;
        }
        if pull.1 > mins.1 {
            mins.1 = pull.1;
        }
        if pull.2 > mins.2 {
            mins.2 = pull.2;
        }
    }

    return mins;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input: &str =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result: u32 = 2286;
        assert_eq!(result, process(input));
    }

    #[test]
    fn test_game_from_str() {
        let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game: Game = Game { id: 1, pulls: vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)], min_colours: (4, 2, 6) };
        assert_eq!(game, Game::from_str(input));
    }

    #[test]
    fn test_parse_pull() {
        let pull: Vec<&str> = vec![" 3 blue", " 4 red"];
        let result: (u32, u32, u32) = (4, 0, 3);
        assert_eq!(result, parse_pull(pull));
    }

    #[test]
    fn test_validate_pulls() {
        let input: Vec<(u32, u32, u32)> = vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)];
        assert_eq!((4, 2, 6), get_mins(&input));
    }
}
