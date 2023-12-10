const START: char = 'S';
const NONE: char = '.';
const VERT: char = '|';
const HORI: char = '-';
const NE: char = 'L';
const NW: char = 'J';
const SW: char = '7';
const SE: char = 'F';

const UP: u8 = 0;
const RIGHT: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 3;

pub fn process(input: &str) -> u64 {
    // Find furthest point from S by following pipes
    let pipe_loop: u64 = find_loop(input);
    let result: u64 = pipe_loop / 2;
    return result;
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct PipeSect {
    x: usize,
    y: usize,
    pipe_type: char,
}
impl PipeSect {
    fn follow_from(&self, dir: u8) -> u8 {
        match self.pipe_type {
            START => panic!("Hit START"),
            NONE => panic!("Hit NONE"),
            VERT => return dir,
            HORI => return dir,
            NE => if dir == LEFT { return UP } else { return RIGHT },
            NW => if dir == RIGHT { return UP } else { return LEFT },
            SE => if dir == LEFT { return DOWN } else { return RIGHT },
            SW => if dir == RIGHT { return DOWN } else { return LEFT },
            _ => panic!("Invalid pipe type"),
        }
    }
}

fn find_loop(input: &str) -> u64 {
    let map: Vec<Vec<char>> = input.lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let start: PipeSect = find_start(input);
    let mut pipe_loop: u64 = 1;

    for i in UP..=LEFT {
        if pipe_loop == 1 {
            match i {
                UP => if start.y != 0 { pipe_loop += walk_loop(&map, start, UP) },
                RIGHT => if start.x != map[start.y].len() { pipe_loop += walk_loop(&map, start, RIGHT) },
                DOWN => if start.y != (map.len() - 1) { pipe_loop += walk_loop(&map, start, DOWN) },
                LEFT => if start.x != 0 { pipe_loop += walk_loop(&map, start, LEFT) },
                _ => panic!("Invalid direction"),
            }
        }
    }

    return pipe_loop;
}

fn find_start(input: &str) -> PipeSect {
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            if char == START {
                let start: PipeSect = PipeSect { x, y, pipe_type: START };
                return start;
            }
        }
    }

    panic!("Couldn't find start location");
}

fn walk_loop(map: &Vec<Vec<char>>, position: PipeSect, direction: u8) -> u64 {
    let mut pipe_loop: u64 = 0;
    let new_pos: (usize, usize) = match direction {
        UP => (position.x, position.y - 1),
        DOWN => (position.x, position.y + 1),
        RIGHT => (position.x + 1, position.y),
        LEFT => (position.x - 1, position.y),
        _ => panic!("Invalid direction given"),
    };
    let pipe_sect: PipeSect = PipeSect { x: new_pos.0, y: new_pos.1, pipe_type: map[new_pos.1][new_pos.0] };

    if pipe_sect.pipe_type == START {
        pipe_loop += 1;
        return pipe_loop;
    }

    if pipe_sect.pipe_type != NONE {
        pipe_loop += 1;
    }

    match pipe_sect.pipe_type {
        START => panic!("Hit START"),
        NONE => {
            pipe_loop = 0;
            return pipe_loop;
        },
        VERT => pipe_loop += walk_loop(map, pipe_sect, pipe_sect.follow_from(direction)),
        HORI => pipe_loop += walk_loop(map, pipe_sect, pipe_sect.follow_from(direction)),
        NE => pipe_loop += walk_loop(map, pipe_sect, pipe_sect.follow_from(direction)),
        NW => pipe_loop += walk_loop(map, pipe_sect, pipe_sect.follow_from(direction)),
        SE => pipe_loop += walk_loop(map, pipe_sect, pipe_sect.follow_from(direction)),
        SW => pipe_loop += walk_loop(map, pipe_sect, pipe_sect.follow_from(direction)),
        _ => panic!("Invalid pipe type"),
    }

    return pipe_loop;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str =
        "..F7.
         .FJ|.
         SJ.L7
         |F--J
         LJ...";

    #[test]
    fn test_process() {
        assert_eq!(8, process(INPUT));
    }
}
