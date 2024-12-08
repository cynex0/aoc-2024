use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }

            if let Some(v) = antennas.get_mut(&c) {
                v.push((i, j));
            } else {
                antennas.insert(c, vec![(i, j)]);
            }
        }
    }

    antennas
}

fn in_bounds(pos: (i32, i32), rows: usize, cols: usize) -> bool {
    // print!("{},{} is ", pos.0, pos.1);
    // if !(pos.0 >= 0 && pos.0 < rows as i32 && pos.1 >= 0 && pos.1 < cols as i32) {
    //     print!("not ");
    // }
    // println!("in bounds.");
    pos.0 >= 0 && pos.0 < rows as i32 && pos.1 >= 0 && pos.1 < cols as i32
}

fn count_antinodes(map: &HashMap<char, Vec<(usize, usize)>>, rows: usize, cols: usize) -> usize {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for k in map.keys() {
        if let Some(poss) = map.get(&k) {
            for (i, pos_i) in poss.iter().enumerate() {
                for pos_j in poss.iter().skip(i + 1) {
                    let (dy, dx): (i32, i32) = (
                        pos_i.0 as i32 - pos_j.0 as i32,
                        pos_i.1 as i32 - pos_j.1 as i32,
                    );
                    let anti_i = (pos_i.0 as i32 + dy, pos_i.1 as i32 + dx);
                    let anti_j = (pos_j.0 as i32 - dy, pos_j.1 as i32 - dx);
                    if in_bounds(anti_i, rows, cols) {
                        antinodes.insert((anti_i.0 as usize, anti_i.1 as usize));
                    }
                    if in_bounds(anti_j, rows, cols) {
                        antinodes.insert((anti_j.0 as usize, anti_j.1 as usize));
                    }
                }
            }
        }
    }

    antinodes.iter().count()
}

fn main() {
    let input = include_str!("input");
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let rows = input.lines().count();
    let cols = input.lines().map(|x| x.chars().count()).max().unwrap_or(0);

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }

            if let Some(v) = antennas.get_mut(&c) {
                v.push((i, j));
            } else {
                antennas.insert(c, vec![(i, j)]);
            }
        }
    }
    let ans_1 = count_antinodes(&antennas, rows, cols);
    println!("Part 1 answer: {ans_1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "...a....\n\
                     .a....b.\n\
                     ...b....\n\
                     abc123..";
        let input = parse_input(input);
        let a = input.get(&'a');
        let b = input.get(&'b');
        let c = input.get(&'c');
        let ones = input.get(&'1');
        let twos = input.get(&'2');
        let threes = input.get(&'3');

        assert_eq!(3, a.unwrap().len());
        assert!(a.unwrap().contains(&(0, 3)));
        assert!(a.unwrap().contains(&(1, 1)));
        assert!(a.unwrap().contains(&(3, 0)));

        assert_eq!(3, b.unwrap().len());
        assert!(b.unwrap().contains(&(1, 6)));
        assert!(b.unwrap().contains(&(2, 3)));
        assert!(b.unwrap().contains(&(3, 1)));

        assert_eq!(1, c.unwrap().len());
        assert!(c.unwrap().contains(&(3, 2)));

        assert_eq!(1, ones.unwrap().len());
        assert!(ones.unwrap().contains(&(3, 3)));

        assert_eq!(1, twos.unwrap().len());
        assert!(twos.unwrap().contains(&(3, 4)));

        assert_eq!(1, threes.unwrap().len());
        assert!(threes.unwrap().contains(&(3, 5)));
    }

    #[test]
    fn counting() {
        let input = "..........\n\
                     ..........\n\
                     ..........\n\
                     ....a.....\n\
                     ........a.\n\
                     .....a....\n\
                     ..........\n\
                     ......A...\n\
                     ..........\n\
                     ..........";
        let map = parse_input(input);

        assert_eq!(
            4,
            count_antinodes(
                &map,
                input.lines().count(),
                input.lines().map(|x| x.chars().count()).max().unwrap_or(0)
            )
        );
    }

    #[test]
    fn example() {
        let input = "............\n\
                    ........0...\n\
                    .....0......\n\
                    .......0....\n\
                    ....0.......\n\
                    ......A.....\n\
                    ............\n\
                    ............\n\
                    ........A...\n\
                    .........A..\n\
                    ............\n\
                    ............";
        let map = parse_input(input);

        assert_eq!(
            14,
            count_antinodes(
                &map,
                input.lines().count(),
                input.lines().map(|x| x.chars().count()).max().unwrap_or(0)
            )
        );
    }
}
