use std::collections::HashMap;

fn concat(a: u64, b: u64) -> u64 {
    let b_digits = b.ilog10() as u32 + 1;
    a * 10_u64.pow(b_digits) + b
}

fn backtrack_1(target: u64, nums: &Vec<u64>, current: u64, index: usize) -> bool {
    if index == nums.len() {
        return current == target;
    }
    if current > target {
        return false;
    }

    let trying_number = current;
    // try adding
    backtrack_1(target, &nums, trying_number + nums[index], index + 1) ||
    // try multiplying
    backtrack_1(target, &nums, trying_number * nums[index], index + 1)
}

fn backtrack_2(target: u64, nums: &Vec<u64>, current: u64, index: usize) -> bool {
    if index == nums.len() {
        return current == target;
    }
    if current > target {
        return false;
    }

    let trying_number = current;
    // try adding
    backtrack_2(target, &nums, trying_number + nums[index], index + 1) ||
    // try multiplying
    backtrack_2(target, &nums, trying_number * nums[index], index + 1) ||
    // try concatenating
    backtrack_2(target, &nums, concat(trying_number, nums[index]), index + 1)
}

fn parse_input(s: &str) -> HashMap<u64, Vec<u64>> {
    let mut values: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in s.lines() {
        if let Some((target, nums)) = line.split_once(": ") {
            println!("{}", target);
            println!("{}", nums);
            values.insert(
                target.parse().unwrap(),
                nums.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
        }
    }

    values
}

fn main() {
    let input = parse_input(include_str!("input"));

    let sum_1: u64 = input
        .iter()
        .filter_map(|(target, nums)| {
            if backtrack_1(*target, nums, nums[0], 1) {
                Some(target)
            } else {
                None
            }
        })
        .into_iter()
        .sum();

    let sum_2: u64 = input
        .iter()
        .filter_map(|(target, nums)| {
            if backtrack_2(*target, nums, nums[0], 1) {
                Some(target)
            } else {
                None
            }
        })
        .into_iter()
        .sum();

    println!("Part 1 answer: {sum_1}");
    println!("Part 2 answer: {sum_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test_input");

    #[test]
    fn example() {
        let input = parse_input(INPUT);
        let mut sum_1 = 0;
        let mut sum_2 = 0;

        for (target, nums) in input {
            if backtrack_1(target, &nums, nums[0], 1) {
                sum_1 += target;
            }
            if backtrack_2(target, &nums, nums[0], 1) {
                sum_2 += target;
            }
        }

        assert_eq!(3749, sum_1);
        assert_eq!(11387, sum_2);
    }

    #[test]
    fn concatenation() {
        assert_eq!(12345, concat(1, 2345));
        assert_eq!(12345, concat(12, 345));
        assert_eq!(12345, concat(123, 45));
        assert_eq!(12345, concat(1234, 5));
    }
}
