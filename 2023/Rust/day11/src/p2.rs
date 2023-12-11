const GALAXY: char = '#';
const EMPTY: char = '.';

const GALAXY_BYTE: u8 = 1;
const EMPTY_BYTE: u8 = 0;

pub fn process(input: &str) -> u64 {
    // get every galaxy location
    // find shortest path from each to eachother
    // sum the paths
    let expanded_map: Map = expand(input);
    let galaxies: Vec<Galaxy> = find_galaxies(&expanded_map);

    let mut routes: Vec<u64> = vec![];
    for i in 0..galaxies.len() {
        let origin: Galaxy = galaxies[i];
        for i_dest in i..galaxies.len() {
            let destination: Galaxy = galaxies[i_dest];
            let route: u64 = find_route(&origin, &destination);
            routes.push(route)
        }
    }

    let sum_of_routes: u64 = routes.iter().sum();
    return sum_of_routes;
}


#[derive(Debug, PartialEq, Clone, Copy)]
struct Galaxy {
    x: usize,
    y: usize,
}
impl Galaxy {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Map {
    base: Vec<Vec<u8>>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}
impl Map {
    fn new(base: Vec<Vec<u8>>, empty_rows: Vec<usize>, empty_cols: Vec<usize>) -> Self {
        Self {
            base,
            empty_rows,
            empty_cols,
        }
    }
}

fn expand(input: &str) -> Map {
    let expanded_map: Vec<Vec<u8>> = input
        .lines()
        .map(|line|
             line.trim().chars()
             .map(|char| match char {
                 GALAXY => GALAXY_BYTE,
                 EMPTY => EMPTY_BYTE,
                 _ => panic!("Invalid char"),
             })
             .collect()
             )
        .collect();

    // Find Empty Rows
    let mut empty_rows: Vec<usize> = vec![];
    for row_index in 0..expanded_map.len() {
        if !expanded_map[row_index].contains(&GALAXY_BYTE) {
            empty_rows.push(row_index);
        }
    }

    // Find Empty Columns
    let mut empty_cols: Vec<usize> = vec![];
    for char_index in 0..expanded_map[0].len() {
        let mut col: Vec<u8> = vec![];
        for line in &expanded_map {
            col.push(line[char_index]);
        }
        if !col.contains(&GALAXY_BYTE) {
            empty_cols.push(char_index);
        }
    }

    let map: Map = Map::new(expanded_map, empty_rows, empty_cols);

    return map;
}

fn find_galaxies(map: &Map) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = vec![];
    for y in 0..map.base.len() {
        let mut expanded_y: usize = y;
        for row in &map.empty_rows {
            if y > *row {
                // Add 1000000 empty rows
                expanded_y += 999999;
            }
        }
        for x in 0..map.base[y].len() {
            let mut expanded_x: usize = x;
            for col in &map.empty_cols {
                if x > *col {
                    expanded_x += 999999;
                }
            }

            if map.base[y][x] == GALAXY_BYTE {
                let galaxy: Galaxy = Galaxy::new(expanded_x, expanded_y);
                galaxies.push(galaxy);
            }
        }
    }

    return galaxies;
}

fn find_route(origin: &Galaxy, destination: &Galaxy) -> u64 {
    let distance: usize = destination.x.abs_diff(origin.x) + destination.y.abs_diff(origin.y);

    return distance as u64;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str =
        "...#......
         .......#..
         #.........
         ..........
         ......#...
         .#........
         .........#
         ..........
         .......#..
         #...#.....";

    #[test]
    fn test_process() {
        assert_eq!(82000210, process(INPUT));
    }

    #[test]
    fn test_find_route() {
        let origin: Galaxy = Galaxy::new(1, 6);
        let destination: Galaxy = Galaxy::new(5, 11);
        assert_eq!(9, find_route(&origin, &destination));
    }
}
