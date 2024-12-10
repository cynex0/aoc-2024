use std::collections::{HashSet, LinkedList};

fn parse_input(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_valid_directions(pos: (usize, usize), rows: usize, cols: usize) -> Vec<(i32, i32)> {
    let mut dirs = Vec::new();
    if pos.0 > 0 {
        dirs.push((-1, 0));
    }
    if pos.0 < rows - 1 {
        dirs.push((1, 0));
    }
    if pos.1 > 0 {
        dirs.push((0, -1));
    }
    if pos.1 < cols - 1 {
        dirs.push((0, 1));
    }

    dirs
}

fn count_paths(map: &Vec<Vec<u32>>, start_pos: (usize, usize)) -> usize {
    // DFS
    let mut paths = 0;
    let rows = map.len();
    let cols = map[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: LinkedList<(usize, usize)> = LinkedList::new();
    stack.push_back(start_pos);

    while let Some(pos) = stack.pop_back() {
        visited.insert(pos);

        let curr = map[pos.0][pos.1];
        if curr == 9 {
            paths += 1;
            continue;
        }

        for dir in get_valid_directions(pos, rows, cols).iter() {
            let row = ((pos.0 as i32) + dir.0) as usize;
            let col = ((pos.1 as i32) + dir.1) as usize;

            if visited.contains(&(row, col)) {
                continue;
            }

            if let Some(v) = map[row][col].checked_sub(curr) {
                if v == 1 {
                    stack.push_back((row, col));
                }
            }
        }
    }

    paths
}

fn main() {
    let input = include_str!("input");
    let map = parse_input(input);
    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, n) in row.iter().enumerate() {
            if *n == 0 {
                sum += count_paths(&map, (i, j))
            }
        }
    }
    println!("Part 1 answer: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directions() {
        let directions = get_valid_directions((0, 0), 10, 10);
        assert_eq!(2, directions.len());
        assert!(directions.contains(&(1, 0)) && directions.contains(&(0, 1)));

        let directions = get_valid_directions((9, 9), 10, 10);
        assert_eq!(2, directions.len());
        assert!(directions.contains(&(-1, 0)) && directions.contains(&(0, -1)));

        let directions = get_valid_directions((0, 9), 10, 10);
        assert_eq!(2, directions.len());
        assert!(directions.contains(&(1, 0)) && directions.contains(&(0, -1)));

        let directions = get_valid_directions((9, 0), 10, 10);
        assert_eq!(2, directions.len());
        assert!(directions.contains(&(-1, 0)) && directions.contains(&(0, 1)));

        let directions = get_valid_directions((5, 0), 10, 10);
        assert_eq!(3, directions.len());
        assert!(
            directions.contains(&(1, 0))
                && directions.contains(&(-1, 0))
                && directions.contains(&(0, 1))
        );

        let directions = get_valid_directions((0, 5), 10, 10);
        assert_eq!(3, directions.len());
        assert!(
            directions.contains(&(1, 0))
                && directions.contains(&(0, 1))
                && directions.contains(&(0, -1))
        );

        let directions = get_valid_directions((5, 9), 10, 10);
        assert_eq!(3, directions.len());
        assert!(
            directions.contains(&(1, 0))
                && directions.contains(&(-1, 0))
                && directions.contains(&(0, -1))
        );

        let directions = get_valid_directions((9, 5), 10, 10);
        assert_eq!(3, directions.len());
        assert!(
            directions.contains(&(-1, 0))
                && directions.contains(&(0, 1))
                && directions.contains(&(0, -1))
        );

        let directions = get_valid_directions((5, 5), 10, 10);
        assert_eq!(4, directions.len());
        assert!(
            directions.contains(&(-1, 0))
                && directions.contains(&(1, 0))
                && directions.contains(&(0, 1))
                && directions.contains(&(0, -1))
        );
    }
}
