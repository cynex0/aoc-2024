use regex::Regex;

fn part_1(a: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for (_, [op1, op2]) in re.captures_iter(a).map(|x| x.extract()) {
        sum += op1.parse::<i64>().unwrap() * op2.parse::<i64>().unwrap();
    }

    sum
}

fn part_2(a: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;

    // i will now try to explain this bc i dont even know what i wrote

    // for all the captures
    for cap in re.captures_iter(a) {
        // if the 0th element of the capture (the whole capture) exists
        if let Some(command) = cap.get(0) {
            // match the whole capture
            match command.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                // all other
                _ => {
                    if enabled {
                        match (cap.get(1), cap.get(2)) {
                            // if both the 1st and the 2nd capture groupt exist
                            (Some(op1), Some(op2)) => {
                                sum += op1.as_str().parse::<i64>().unwrap()
                                    * op2.as_str().parse::<i64>().unwrap()
                            }
                            // else, basically
                            _ => panic!("Not enough operands"),
                        };
                    }
                }
            }
        }
    }
    // i have a feeling this is now how its done...

    sum
}

fn main() {
    let input = include_str!("input");
    println!("Part 1 answer: {}", part_1(&input));
    println!("Part 2 answer: {}", part_2(&input));
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part_1("mul(44,46)"), 2024);
    }

    #[test]
    fn example2() {
        assert_eq!(
            part_1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn should_ignore() {
        assert_eq!(0, part_1("mul(4*"));
        assert_eq!(0, part_1("mul(6,9!"));
        assert_eq!(0, part_1("?(12,34)"));
        assert_eq!(0, part_1("mul ( 2 , 4 )"));
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part_2("mul(44,46)"), 2024);
    }

    #[test]
    fn example2() {
        assert_eq!(
            part_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }

    #[test]
    fn should_ignore() {
        assert_eq!(0, part_2("mul(4*"));
        assert_eq!(0, part_2("mul(6,9!"));
        assert_eq!(0, part_2("?(12,34)"));
        assert_eq!(0, part_2("mul ( 2 , 4 )"));
    }
}
