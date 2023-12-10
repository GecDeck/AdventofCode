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

const FLIP_IN_LOOP: char = '1';

pub fn process(input: &str) -> u64 {
    // Find furthest point from S by following pipes
    let pipe_loop: u64 = find_inside(input);
    let result: u64 = pipe_loop;
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

    fn connects_to(&self, other: PipeSect) -> bool {
        match self.pipe_type {
            VERT => if other.y == self.y + 1 || other.y == self.y - 1 { return true },
            HORI => if other.x == self.x + 1 || other.x == self.x - 1 { return true },
            NE => if other.y == self.y - 1 || other.x == self.x + 1 { return true },
            NW => if other.y == self.y - 1 || other.x == self.x - 1 { return true },
            SE => if other.y == self.y + 1 || other.x == self.x + 1 { return true },
            SW => if other.y == self.y + 1 || other.x == self.x - 1 { return true },
            START => return true,
            NONE => return false,
            _ => panic!("Invalid pipe type"),
        }
        return false;
    }
}

fn find_inside(input: &str) -> u64 {
    let mut map: Vec<Vec<char>> = input.lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let start: PipeSect = find_start(input);

    let mut direction: u8 = 5;
    for i in UP..=LEFT {
        if direction == 5 {
            match i {
                UP => if start.y != 0 && walk_loop(&map, start, UP) != 0 { direction = UP },
                RIGHT => if start.x != map[start.y].len() && walk_loop(&map, start, RIGHT) != 0 { direction = RIGHT },
                DOWN => if start.y != (map.len() - 1) && walk_loop(&map, start, DOWN) != 0 { direction = DOWN },
                LEFT => if start.x != 0 && walk_loop(&map, start, LEFT) != 0 { direction = LEFT },
                _ => panic!("Invalid direction"),
            }
        }
    }

    // Stack overflow below!
    let came_from: u8 = replace_loop(&mut map, start, direction);
    // Stack overflow above!
    let start_char: char = match direction {
        UP => match came_from {
            DOWN => '|',
            LEFT => 'J',
            RIGHT => 'L',
            _ => panic!("Unhandled exception")
        },
        RIGHT => match came_from {
            UP => 'L',
            DOWN => 'F',
            LEFT => '-',
            _ => panic!("Unhandled exception")
        },
        DOWN => match came_from {
            UP => '|',
            RIGHT => 'F',
            LEFT => '7',
            _ => panic!("Unhandled exception")
        },
        LEFT => match came_from {
            UP => 'J',
            RIGHT => '-',
            DOWN => '7',
            _ => panic!("Unhandled exception")
        },
        _ => panic!("Unhandled exception")
    };

    map[start.y][start.x] = match start_char {
        START => 'X',
        VERT => '1',
        NE => '1',
        NW => '1',
        HORI => '0',
        SE => '0',
        SW => '0',
        NONE => panic!("Should not come across a ."),
        _ => panic!("Invalid pipe type"),
    };

    let mut is_inside: bool = false;
    let mut inside_sum: u64 = 0;
    for line in map {
        for char in line {
            if char == FLIP_IN_LOOP {
                is_inside = !is_inside;
            }
            if is_inside && char != '0' && char != '1' {
                inside_sum += 1;
            }
        }
    }
    // walk map here checking for inside or outside

    return inside_sum;
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

    if !pipe_sect.connects_to(position) {
        return 0;
    }

    if pipe_sect.pipe_type == START {
        pipe_loop += 1;
        return pipe_loop;
    }

    if pipe_sect.pipe_type != NONE {
        pipe_loop += 1;
    }

    match pipe_sect.pipe_type {
        START => panic!("Hit START"),
        NONE => { return 0 },
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

fn replace_loop(map: &mut Vec<Vec<char>>, position: PipeSect, direction: u8) -> u8 {
    let new_pos: (usize, usize) = match direction {
        UP => (position.x, position.y - 1),
        DOWN => (position.x, position.y + 1),
        RIGHT => (position.x + 1, position.y),
        LEFT => (position.x - 1, position.y),
        _ => panic!("Invalid direction given"),
    };
    let pipe_sect: PipeSect = PipeSect { x: new_pos.0, y: new_pos.1, pipe_type: map[new_pos.1][new_pos.0] };

    map[pipe_sect.y][pipe_sect.x] = match pipe_sect.pipe_type {
        START => 'X',
        VERT => '1',
        NE => '1',
        NW => '1',
        HORI => '0',
        SE => '0',
        SW => '0',
        NONE => panic!("Should not come across a ."),
        _ => panic!("Invalid pipe type"),
    };

    if pipe_sect.pipe_type == START {
        return match direction {
            UP => DOWN,
            RIGHT => LEFT,
            DOWN => UP,
            LEFT => RIGHT,
            _ => panic!("Invalid direction"),
        }
    }

    let prev_dir: u8 = match pipe_sect.pipe_type {
        START => panic!("Hit START"),
        NONE => { return 255; },
        VERT => replace_loop(map, pipe_sect, pipe_sect.follow_from(direction)),
        HORI => replace_loop(map, pipe_sect, pipe_sect.follow_from(direction)),
        NE => replace_loop(map, pipe_sect, pipe_sect.follow_from(direction)),
        NW => replace_loop(map, pipe_sect, pipe_sect.follow_from(direction)),
        SE => replace_loop(map, pipe_sect, pipe_sect.follow_from(direction)),
        SW => replace_loop(map, pipe_sect, pipe_sect.follow_from(direction)),
        _ => panic!("Invalid pipe type"),
    };

    return prev_dir;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str =
        "FF7FSF7F7F7F7F7F---7
         L|LJ||||||||||||F--J
         FL-7LJLJ||||||LJL-77
         F--JF--7||LJLJ7F7FJ-
         L---JF-JLJ.||-FJLJJ7
         |F|F-JF---7F7-L7L|7|
         |FFJF7L7F-JF7|JL---7
         7-L-JL7||F7|L7F-7F7|
         L.L7LFJ|||||FJL7||LJ
         L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_process() {
        assert_eq!(10, process(INPUT));
    }
}
