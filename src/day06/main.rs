use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Position {
    fn get_dir_vec(&self) -> (i32, i32) {
        match self.dir {
            Direction::RIGHT => return (1, 0),
            Direction::LEFT => return (-1, 0),
            Direction::UP => return (0, -1),
            Direction::DOWN => return (0, 1),
        }
    }

    fn turn_right(&mut self) {
        match self.dir {
            Direction::RIGHT => self.dir = Direction::DOWN,
            Direction::LEFT => self.dir = Direction::UP,
            Direction::UP => self.dir = Direction::RIGHT,
            Direction::DOWN => self.dir = Direction::LEFT,
        }
    }

    fn update(&mut self) {
        let (_x, _y) = self.get_dir_vec();
        self.x += _x;
        self.y += _y;
    }
}

fn move_guard(pos: &mut Position, map: &Vec<Vec<char>>, rows: usize, cols: usize) -> bool {
    let (x, y) = (pos.x, pos.y);
    let (dx, dy) = (pos.get_dir_vec().0, pos.get_dir_vec().1);

    if x > 0 && x < cols as i32 - 1 && y > 0 && y < rows as i32 - 1 {
        let next = map[(y + dy) as usize][(x + dx) as usize];
        if next == '#' {
            pos.turn_right();
        }
    }

    pos.update();
    pos.x >= 0 && pos.x < cols as i32 && pos.y >= 0 && pos.y < rows as i32
}

fn pathfind(map: &mut Vec<Vec<char>>, pos: &mut Position) -> HashSet<(usize, usize)> {
    let rows = map.len();
    let cols = map[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((pos.x as usize, pos.y as usize));

    loop {
        if !move_guard(pos, map, rows, cols) {
            break;
        }
        visited.insert((pos.x as usize, pos.y as usize));
    }

    visited
}

fn get_start_pos(map: &Vec<Vec<char>>) -> Position {
    let mut pos = Position {
        x: (0),
        y: (0),
        dir: (Direction::UP),
    };

    'outer: for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                pos.x = j as i32;
                pos.y = i as i32;
                break 'outer;
            }
        }
    }

    pos
}

fn main() {
    let input = include_str!("input");
    let mut map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let start_pos = get_start_pos(&map);
    let visited = pathfind(&mut map, &mut start_pos.clone());
    println!("Part 1 answer: {}", visited.iter().count());
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn example() {
        let str = "....#.....\n\
                   .........#\n\
                   ..........\n\
                   ..#.......\n\
                   .......#..\n\
                   ..........\n\
                   .#..^.....\n\
                   ........#.\n\
                   #.........\n\
                   ......#...";

        let mut map: Vec<Vec<char>> = str.lines().map(|x| x.chars().collect()).collect();
        let mut pos = get_start_pos(&map);
        let visited = pathfind(&mut map, &mut pos);
        assert_eq!(41, visited.iter().count());
    }
}
