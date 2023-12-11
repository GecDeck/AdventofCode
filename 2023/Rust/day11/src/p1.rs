const GALAXY: char = '#';
const EMPTY: char = '.';

pub fn process(input: &str) -> u32 {
    // double all lines that need to be doubled
    // get every galaxy location
    // find shortest path from each to eachother
    // sum the paths
    let expanded_map: Vec<Vec<char>> = expand(input);
    let galaxies: Vec<Galaxy> = find_galaxies(&expanded_map);

    let mut routes: Vec<u32> = vec![];
    for i in 0..galaxies.len() {
        let origin: Galaxy = galaxies[i];
        for i_dest in i..galaxies.len() {
            let destination: Galaxy = galaxies[i_dest];
            let route: u32 = find_route(&expanded_map, &origin, &destination);
            routes.push(route)
        }
    }

    let sum_of_routes: u32 = routes.iter().sum();
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

fn expand(input: &str) -> Vec<Vec<char>> {
    let mut expanded_map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    // Find Empty Rows
    let mut empty_rows: Vec<usize> = vec![];
    for row_index in 0..expanded_map.len() {
        if !expanded_map[row_index].contains(&GALAXY) {
            empty_rows.push(row_index);
        }
    }

    // Find Empty Columns
    let mut empty_cols: Vec<usize> = vec![];
    for char_index in 0..expanded_map[0].len() {
        let mut col: Vec<char> = vec![];
        for line in &expanded_map {
            col.push(line[char_index]);
        }
        if !col.contains(&GALAXY) {
            empty_cols.push(char_index);
        }
    }

    // Expand Rows
    let mut expanded_rows: usize = 0;
    // Need to increment this to accomadate for the location
    //  of empty rows being moved by adding new ones
    for row in empty_rows {
        let mut empty_row: Vec<char> = vec![];
        for _ in 0..expanded_map[0].len() {
            empty_row.push(EMPTY);
        }

        expanded_map.insert(row + expanded_rows, empty_row);
        expanded_rows += 1;
    }

    // Expand Columns
    let mut expanded_col: usize = 0;
    for col in empty_cols {
        for i in 0..expanded_map.len() {
            expanded_map[i].insert(col + expanded_col, EMPTY);
        }

        expanded_col += 1;
    }

    return expanded_map;
}

fn find_galaxies(map: &Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = vec![];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == GALAXY {
                galaxies.push(Galaxy::new(x, y));
            }
        }
    }

    return galaxies;
}

fn find_route(map: &Vec<Vec<char>>, origin: &Galaxy, destination: &Galaxy) -> u32 {
    let distance: usize = destination.x.abs_diff(origin.x) + destination.y.abs_diff(origin.y);

    return distance as u32;
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
    const INPUT_EXPANDED: &str =
        "....#........
         .........#...
         #............
         .............
         .............
         ........#....
         .#...........
         ............#
         .............
         .............
         .........#...
         #....#.......";

    #[test]
    fn test_process() {
        assert_eq!(374, process(INPUT));
    }

    #[test]
    fn test_expand() {
        let map: Vec<Vec<char>> = INPUT_EXPANDED
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();
        assert_eq!(map, expand(INPUT));
    }

    #[test]
    fn test_find_galaxies() {
        let galaxies: Vec<Galaxy> = vec![
            Galaxy::new(4, 0), Galaxy::new(9, 1), Galaxy::new(0, 2),
            Galaxy::new(8, 5), Galaxy::new(1, 6), Galaxy::new(12, 7),
            Galaxy::new(9, 10), Galaxy::new(0, 11), Galaxy::new(5, 11),
        ];
        assert_eq!(galaxies, find_galaxies(&expand(INPUT)));
    }

    #[test]
    fn test_find_route() {
        let origin: Galaxy = Galaxy::new(1, 6);
        let destination: Galaxy = Galaxy::new(5, 11);
        let map: Vec<Vec<char>> = expand(INPUT);
        assert_eq!(9, find_route(&map, &origin, &destination));
    }
}
