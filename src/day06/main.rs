enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

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
        print!("Turned right ");
    }

    fn update(&mut self) {
        let (_x, _y) = self.get_dir_vec();
        self.x += _x;
        self.y += _y;
        print!("Moved by {_x},{_y}");
    }
}

fn pathfind(map: &mut Vec<Vec<char>>, pos: &mut Position) {
    let rows = map.len();
    let cols = map[0].len();

    print!("At {},{}: ", pos.x, pos.y);

    while pos.x >= 0 && pos.x < cols as i32 && pos.y >= 0 && pos.y < rows as i32 {
        let (x, y) = (pos.x as usize, pos.y as usize);
        map[y][x] = 'X';

        match pos.dir {
            Direction::UP => {
                if y >= 1 && map[y - 1][x] == '#' {
                    pos.turn_right();
                }
            }
            Direction::DOWN => {
                if y < rows - 1 && map[y + 1][x] == '#' {
                    pos.turn_right();
                }
            }
            Direction::LEFT => {
                if x >= 1 && map[y][x - 1] == '#' {
                    pos.turn_right();
                }
            }
            Direction::RIGHT => {
                if x < cols - 1 && map[y][x + 1] == '#' {
                    pos.turn_right();
                }
            }
        }
        pos.update();
        println!();
    }
}

fn main() {
    let input = include_str!("input");
    let mut map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    println!("Rows: {}, Cols: {}", map.len(), map[0].len());

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
    pathfind(&mut map, &mut pos);
    println!(
        "Part 1 answer: {}",
        map.iter().flatten().filter(|&&x| x == 'X').count()
    );
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

        pathfind(&mut map, &mut pos);
        for line in map.iter() {
            for c in line {
                print!("{c}");
            }
            println!();
        }

        assert_eq!(41, map.iter().flatten().filter(|&&x| x == 'X').count());
    }
}
