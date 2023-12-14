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
    // part 1
    let lines = read_lines("input");
    let re = Regex::new("[0-9]").unwrap();
    let list = lines.iter().map(|l| re.find(l).unwrap().as_str().to_owned() +
        &re.find(&l.chars().rev().collect::<String>()).unwrap().as_str().chars().rev().collect::<String>())
        .map(|s| s.trim().parse::<i64>().unwrap());
    let res = list.reduce(|acc, e| acc + e).unwrap();
    println!("part 1 ans: {}", res);


    // part 2
    let lines = read_lines("input");

    let list = lines
        .iter()
        .map(|l| {
            let l = l.replace("one", "one1one");
            let l = l.replace("two", "two2two");
            let l = l.replace("three", "three3three");
            let l = l.replace("four", "four4four");
            let l = l.replace("five", "five5five");
            let l = l.replace("six", "six6six");
            let l = l.replace("seven", "seven7seven");
            let l = l.replace("eight", "eight8eight");
            l.replace("nine", "nine9nine")
        });

    list.clone().for_each(|l| println!("{l}"));

    let list = list.map(|l| re.find(&l).unwrap().as_str().to_owned() +
        &re.find(&l.chars().rev().collect::<String>()).unwrap().as_str().chars().rev().collect::<String>())
        .map(|s| s.trim().parse::<i64>().unwrap());
    let res = list.reduce(|acc, e| acc + e).unwrap();

    
    println!("part 2 ans: {}", res);
}
