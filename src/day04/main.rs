fn search_omnidir(s: &str, t: &str) -> i32 {
    let mut counter = 0;
    let target: Vec<char> = t.chars().collect();
    let t_len = target.len();

    let mat: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let cols = mat[0].len();
    let rows = mat.len();

    for r in 0..rows {
        for c in 0..cols {
            if mat[r][c] != target[0] {
                continue;
            }

            // horizontal right
            if c + t_len <= cols {
                if (1..t_len).all(|i| mat[r][c + i] == target[i]) {
                    counter += 1;
                }

                // down (diag)
                if r + t_len <= rows {
                    if (1..t_len).all(|i| mat[r + i][c + i] == target[i]) {
                        counter += 1;
                    }
                }

                // up (diag)
                if r >= t_len - 1 {
                    if (1..t_len).all(|i| mat[r - i][c + i] == target[i]) {
                        counter += 1;
                    }
                }
            }
            // horizontal left
            if c >= t_len - 1 {
                if (1..t_len).all(|i| mat[r][c - i] == target[i]) {
                    counter += 1;
                }

                // down (diag)
                if r + t_len <= rows {
                    if (1..t_len).all(|i| mat[r + i][c - i] == target[i]) {
                        counter += 1;
                    }
                }

                // up (diag)
                if r >= t_len - 1 {
                    if (1..t_len).all(|i| mat[r - i][c - i] == target[i]) {
                        counter += 1;
                    }
                }
            }

            // vertical down
            if r + t_len <= rows {
                if (1..t_len).all(|i| mat[r + i][c] == target[i]) {
                    counter += 1;
                }
            }
            // vertical up
            if r >= t_len - 1 {
                if (1..t_len).all(|i| mat[r - i][c] == target[i]) {
                    counter += 1;
                }
            }
        }
    }
    counter
}

// tried to generalize it,
// very close to the correct answer (off by 6!!!!), idk why
fn search_omnidir_alt(s: &str, t: &str) -> i32 {
    let mut counter = 0;
    let target: Vec<char> = t.chars().collect();
    let t_len = target.len();

    let mat: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let cols = mat[0].len();
    let rows = mat.len();

    let dirs = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (1, 1),
        (-1, 1),
    ];
    for row in 0..rows {
        for col in 0..cols {
            for (dr, dc) in dirs {
                if (0..t_len).all(|i| {
                    let r = row as isize + dr * i as isize; // r index to check
                    let c = col as isize + dc * i as isize; // c index to check
                    r >= 0 && r < rows as isize
                    && c >= 0 && c < cols as isize // in bounds
                    && mat[r as usize][c as usize] == target[i] // chars at a given offset match
                }) {
                    counter += 1;
                    println!(
                        "Found {:?} at ({}, {}) direction ({}, {})",
                        t, row, col, dr, dc
                    );
                };
            }
        }
    }
    counter
}

fn search_xes(s: &str) -> i32 {
    let mut counter = 0;
    let matches = ["MMSS", "MSSM", "SSMM", "SMMS"];

    let mat: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let cols = mat[0].len();
    let rows = mat.len();

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if mat[row][col] == 'A' {
                let surr_chars = vec![
                    mat[row - 1][col - 1],
                    mat[row - 1][col + 1],
                    mat[row + 1][col + 1],
                    mat[row + 1][col - 1],
                ];
                let surr_string: String = surr_chars.into_iter().collect();
                if matches.iter().any(|s| *s == surr_string) {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn main() {
    let input = include_str!("input");
    let sum_matches = search_omnidir(input, "XMAS");
    println!("Part 1 answer: {}", sum_matches);
    println!("Part 2 answer: {}", search_xes(input));
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn example() {
        let input = "MMMSXXMASM\n\
                     MSAMXMSMSA\n\
                     AMXSXMAAMM\n\
                     MSAMASMSMX\n\
                     XMASAMXAMM\n\
                     XXAMMXXAMA\n\
                     SMSMSASXSS\n\
                     SAXAMASAAA\n\
                     MAMMMXMMMM\n\
                     MXMXAXMASX";
        assert_eq!(search_omnidir(input, "XMAS"), 18);
    }

    #[test]
    fn horizontal() {
        let input = "..X....XMAS..X..\n\
                     ...XMASAMX..XMAS";
        assert_eq!(4, search_omnidir(input, "XMAS"));
    }

    #[test]
    fn vertical() {
        let input = "XSX\n\
                     MA.\n\
                     AM.\n\
                     SXX";
        assert_eq!(2, search_omnidir(input, "XMAS"));
    }
}
