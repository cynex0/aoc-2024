use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_distances(a: &[u32], b: &[u32]) -> Vec<u32> {
    assert_eq!(a.len(), b.len());

    let mut distances = Vec::with_capacity(a.len());
    let mut a_sorted = a.to_owned();
    let mut b_sorted = b.to_owned();
    a_sorted.sort_unstable();
    b_sorted.sort_unstable();

    for (a, b) in a_sorted.iter().zip(b_sorted.iter()) {
        distances.push(a.abs_diff(*b));
    }

    distances
}

fn get_similarity(a: &[u32], b: &[u32]) -> u32 {
    let mut freq_map = HashMap::new();
    let mut frequency: u32 = 0;

    for i in b.iter() {
        if !freq_map.contains_key(i) {
            freq_map.insert(i, 1);
        } else {
            freq_map.insert(i, freq_map.get(i).unwrap() + 1);
        }
    }

    for i in a.iter() {
        match freq_map.get(i) {
            Some(v) => frequency += i * v,
            None => (),
        }
    }

    frequency
}

fn main() {
    let path = Path::new("input");
    let file = File::open(&path).expect("Error opening file");

    let mut a: Vec<u32> = Vec::new();
    let mut b: Vec<u32> = Vec::new();

    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        for (i, number) in line.split_whitespace().enumerate() {
            match i {
                0 => a.push(number.parse().unwrap()),
                1 => b.push(number.parse().unwrap()),
                _ => panic!("Too many numbers in line"),
            }
        }
    }

    println!(
        "Part 1 answer: {}",
        get_distances(&a, &b).iter().sum::<u32>()
    );
    println!("Part 2 answer: {}", get_similarity(&a, &b));
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn example() {
        let a = [3, 4, 2, 1, 3, 3];
        let b = [4, 3, 5, 3, 9, 3];
        let distances = get_distances(&a, &b);
        let sum: u32 = distances.iter().sum();
        assert_eq!(sum, 11);
    }

    #[test]
    fn empty() {
        let a = [];
        let b = [];
        let distances = get_distances(&a, &b);
        let sum: u32 = distances.iter().sum();
        assert_eq!(sum, 0);
    }

    #[test]
    #[should_panic]
    fn different_lengths() {
        let a = [1, 2, 3];
        let b = [1, 2];
        get_distances(&a, &b);
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn example() {
        let a = [3, 4, 2, 1, 3, 3];
        let b = [4, 3, 5, 3, 9, 3];
        let similarity = get_similarity(&a, &b);
        assert_eq!(similarity, 31);
    }
}
