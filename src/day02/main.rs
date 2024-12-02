use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn is_safe(a: &[i32]) -> bool {
    let mut diffs: Vec<i32> = Vec::with_capacity(a.len() - 1);
    for window in a.windows(2) {
        let diff = window[1] - window[0];
        if diff.abs() == 0 || diff.abs() > 3 {
            return false;
        }
        diffs.push(diff);
    }

    return diffs.iter().all(|&x| x > 0) || diffs.iter().all(|&x| x < 0);
}

fn main() {
    let path = Path::new("input");
    let file = File::open(&path).expect("Error opening file");

    let lines = BufReader::new(file).lines();
    let mut counter = 0;
    for line in lines.flatten() {
        let mut arr = vec![];
        for number in line.split_whitespace() {
            arr.push(number.parse().unwrap());
        }
        if is_safe(&arr) {
            counter += 1;
        }
    }

    println!("Part 1 answer: {}", counter);
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn example1() {
        let a = [7, 6, 4, 2, 1];
        assert_eq!(true, is_safe(&a));
    }

    #[test]
    fn example2() {
        let a = [1, 2, 7, 8, 9];
        assert_eq!(false, is_safe(&a));
    }

    #[test]
    fn example3() {
        let a = [9, 7, 6, 2, 1];
        assert_eq!(false, is_safe(&a));
    }

    #[test]
    fn example4() {
        let a = [8, 6, 4, 4, 1];
        assert_eq!(false, is_safe(&a));
    }

    #[test]
    fn example5() {
        let a = [1, 3, 6, 7, 9];
        assert_eq!(true, is_safe(&a));
    }
}
