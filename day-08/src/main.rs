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

    // part 2
    let lines = read_lines("input");
    let mut lines = lines.iter();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let mut hm: HashMap<String, (String, String)> = HashMap::new();

    let mut starts: Vec<String> = vec![];

    lines.skip(1).for_each(|l| {
        let replaced = l.replace(" = (", ",").replace(")", "");
        let mut splits = replaced.split(',').map(|s| s.trim());

        let first = splits.next().unwrap().to_string();
        let second = splits.next().unwrap().to_string();
        let third = splits.next().unwrap().to_string();
        let pos = first.clone();
        if pos.ends_with("A") {
            starts.push(pos)
        }
        hm.insert(first, (second, third));
    });

    let len_instr = instructions.len();
    let mut i = 0;
    let steps = starts
        .iter()
        .map(|s| {
            i = 0;
            let mut position = s;
            while !position.ends_with("Z") {
                let instr = instructions[i % len_instr];
                let (left, right) = hm.get(position).unwrap();
                if instr == 'L' {
                    position = left
                } else {
                    position = right
                }

                i += 1;
            }
            i
        })
        .collect::<Vec<_>>();
    let res = lcm(&steps);

    println!("part 2 ans: {}", res);
    assert_eq!(res, 10921547990923);
}

// burrowed from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
