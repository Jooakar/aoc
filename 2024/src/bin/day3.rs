use std::fmt::Display;

use regex::Regex;

fn part1(input: &str) -> impl Display {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [l, r])| l.parse::<i64>().unwrap() * r.parse::<i64>().unwrap())
        .sum::<i64>()
}

fn part2(input: &str) -> impl Display {
    let re = Regex::new(r"don't\(\).*?(do\(\)|\n)").unwrap();
    part1(&re.replace_all(input, ""))
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "ex" {
        let example: &str = include_str!("../../input/3.ex");
        println!("EXAMPLE 1: {}", part1(example));
        println!("EXAMPLE 2: {}", part2(example));
    } else {
        let input: &str = include_str!("../../input/3");
        println!("PART 1: {}", part1(input));
        println!("PART 2: {}", part2(input));
    }
}
