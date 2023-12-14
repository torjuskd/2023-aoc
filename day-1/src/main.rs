use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let lines = read_lines("input");
    let re = Regex::new("[0-9]").unwrap();
    let list: Vec<String> = lines.iter().map(|l| re.find(l).unwrap().as_str().to_owned() +
        &re.find(&l.chars().rev().collect::<String>()).unwrap().as_str().chars().rev().collect::<String>())
            .collect();
    let parsed = list.iter().map(|s| s.trim().parse::<i64>().unwrap());
    let res = parsed.reduce(|acc, e| acc + e).unwrap();
    println!("{}", res)

    //println!("{}", res);


    // 54338
}
