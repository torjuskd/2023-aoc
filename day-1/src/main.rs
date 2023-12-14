use regex::Regex;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn find_first_and_last_digit_string_list(list: Vec<String>) -> Vec<i64> {
    let re = Regex::new("[0-9]").unwrap();
    list.iter()
        .map(|l| {
            re.find(&l).unwrap().as_str().to_owned()
                + &re
                    .find(&l.chars().rev().collect::<String>())
                    .unwrap()
                    .as_str()
                    .chars()
                    .rev()
                    .collect::<String>()
        })
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect()
}

fn main() {
    // part 1
    let lines = read_lines("input");
    let list = find_first_and_last_digit_string_list(lines);
    let res: i64 = list.iter().sum();
    println!("part 1 ans: {}", res);

    // part 2
    let lines = read_lines("input");

    let list = lines.iter().map(|l| {
        l.replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
    });

    let list = find_first_and_last_digit_string_list(list.collect());
    let res: i64 = list.iter().sum();
    println!("part 2 ans: {}", res);
}
