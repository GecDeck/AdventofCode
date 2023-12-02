const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

pub fn process(input: &str) -> u32 {
    let mut games: Vec<Game> = vec![];
    for line in input.lines() {
        let game: Game = Game::from_str(line);
        if game.valid {
            games.push(game);
        }
    }

    let sum_of_ids: u32 = games.iter().map(|game| game.id).sum();
    return sum_of_ids;
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    pulls: Vec<(u32, u32, u32)>,
    // (red, green, blue)
    valid: bool,
}
impl Game {
    fn from_str(input: &str) -> Self {
        let id: u32 = input.replace("Game ", "").trim()
            .split(":").next().expect("getting id of input")
            .trim().parse().expect("parsing id");

        let pulls: Vec<(u32, u32, u32)> = input.split(":").last()
            .expect("getting second half of input")
            .split(";")
            .map( |pull|
                  parse_pull(pull.split(",").collect())
                )
            .collect();

        let valid: bool = validate_pulls(&pulls);

        Self {
            id,
            pulls,
            valid,
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

fn validate_pulls(pulls: &[(u32, u32, u32)]) -> bool {
    // Returns true if all pulls in a game are valid
    for pull in pulls {
        if pull.0 > RED_CUBES || pull.1 > GREEN_CUBES || pull.2 > BLUE_CUBES {
            return false;
        }
    }
    return true;
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
        let result: u32 = 8;
        assert_eq!(result, process(input));
    }

    #[test]
    fn test_game_from_str() {
        let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game: Game = Game { id: 1, pulls: vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)], valid: true };
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
        let input_valid: Vec<(u32, u32, u32)> = vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)];
        assert!(validate_pulls(&input_valid));

        let input_invalid: Vec<(u32, u32, u32)> = vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)];
        assert!(!validate_pulls(&input_invalid));
    }
}
