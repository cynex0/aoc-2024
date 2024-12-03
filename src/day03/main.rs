use regex::Regex;

fn part_1(a: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for (full, [op1, op2]) in re.captures_iter(a).map(|x| x.extract()) {
        sum += op1.parse::<i64>().unwrap() * op2.parse::<i64>().unwrap();
    }

    sum
}

fn main() {
    let input = include_str!("input");
    println!("{}", part_1(&input));
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
