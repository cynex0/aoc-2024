use core::fmt;
use std::collections::HashSet;
use std::{collections::HashMap, num::ParseIntError};

#[derive(Debug)]
enum ParseRuleError {
    InvalidFormat,
    InvalidNumber(ParseIntError),
}

impl fmt::Display for ParseRuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "Expected rule format: 1|2"),
            Self::InvalidNumber(e) => write!(f, "Error while parsing number: {}", e),
        }
    }
}

impl From<ParseIntError> for ParseRuleError {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidNumber(err)
    }
}

fn parse_rule(rule: &str) -> Result<(usize, usize), ParseRuleError> {
    let nums: Vec<&str> = rule.split('|').collect();
    if nums.len() != 2 {
        Err(ParseRuleError::InvalidFormat)
    } else {
        let first = nums[0].parse()?;
        let second = nums[1].parse()?;
        Ok((first, second))
    }
}

fn follows_rules(nums: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool {
    let mut seen = HashSet::new();
    for num in nums {
        if let Some(rule) = rules.get(&num) {
            if rule.iter().any(|e| seen.contains(e)) {
                return false;
            }
        }
        seen.insert(num);
    }

    true
}

fn reorder(nums: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut res = vec![];
    for num in nums {
        if let Some(rule) = rules.get(&num) {
            if let Some(insert_at) = res.iter().position(|x| rule.contains(x)) {
                res.insert(insert_at, *num);
            } else {
                res.push(*num);
            }
        }
    }

    res
}

fn main() {
    let input: Vec<&str> = include_str!("input").split("\n\n").collect();
    if input.len() != 2 {
        panic!("My parsing is wrong again. sigh....");
    }

    let rules_s = input[0];
    let lines = input[1].lines();
    let mut rules_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for rule_s in rules_s.lines() {
        let (first, second) = parse_rule(rule_s).expect("i love parsing...");

        if let Some(v) = rules_map.get_mut(&first) {
            v.push(second);
        } else {
            rules_map.insert(first, vec![second]);
        }
    }

    let mut sum = 0;
    let mut sum_with_reorder = 0;

    for line in lines {
        let nums: Vec<usize> = line
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        if follows_rules(&nums, &rules_map) {
            sum += nums[nums.len() / 2];
        } else {
            let reordered = reorder(&nums, &rules_map);
            sum_with_reorder += reordered[reordered.len() / 2];
        }
    }

    println!("Part 1 answer: {sum}");
    println!("Part 2 answer: {sum_with_reorder}");
}
