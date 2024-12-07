use std::collections::HashMap;

fn backtrack(target: u64, nums: &Vec<u64>, current: u64, index: usize) -> bool {
    if current == target {
        return true;
    }
    if index == nums.len() {
        return current == target;
    }
    if current > target {
        return false;
    }

    let trying_number = current;
    // try adding
    if backtrack(target, &nums, trying_number + nums[index], index + 1) {
        return true;
    }

    // try multiplying
    backtrack(target, &nums, trying_number * nums[index], index + 1)
}

fn main() {
    let input = include_str!("input");
    let mut values: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in input.lines() {
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

    let mut sum = 0;
    for (target, nums) in values {
        if backtrack(target, &nums, nums[0], 1) {
            sum += target;
        }
    }

    println!("Part 1 answer: {sum}");
}
