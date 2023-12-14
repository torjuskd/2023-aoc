use std::{collections::HashMap, fs::read_to_string};

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
    let mut lines = lines.iter();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let mut hm: HashMap<String, (String, String)> = HashMap::new();
    let start = String::from("AAA");
    let end = String::from("ZZZ");

    lines.skip(1).for_each(|l| {
        let replaced = l.replace(" = (", ",").replace(")", "");
        let mut splits = replaced.split(',').map(|s| s.trim());

        let first = splits.next().unwrap().to_string();
        let second = splits.next().unwrap().to_string();
        let third = splits.next().unwrap().to_string();
        hm.insert(first, (second, third));
    });

    let mut position = &start;
    let mut i = 0;
    let len_instr = instructions.len();

    while position != &end {
        //println!("{position} {end}");
        let instr = instructions[i % len_instr];
        //println!("{instr}");
        let (left, right) = hm.get(position).unwrap();
        if instr == 'L' {
            position = left
        } else {
            position = right
        }

        i += 1;
    }

    println!("part 1 ans: {}", i);
    assert_eq!(i, 14429);
}
