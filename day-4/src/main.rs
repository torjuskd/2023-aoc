use std::collections::HashSet;
use std::fs::read_to_string;

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

    let sum: u32 = lines
        .iter()
        .map(|l| {
            let mut game_map: HashSet<i32> = HashSet::new();
            let map = l
                .split(':')
                .skip(1)
                .next()
                .unwrap()
                .split([':', '|', ' '])
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|v| {
                    if game_map.insert(v.parse::<i32>().unwrap()) {
                        0
                    } else {
                        1
                    }
                });

            let dups: u32 = map.sum();
            if dups == 0 || dups == 1 {
                return dups;
            }
            1 * 2_u32.pow(dups - 1)
        })
        .sum();

    println!("part 1 ans: {}", sum);
    assert_eq!(sum, 22897);

    // part 2
    let lines = read_lines("input");

    let num_cards = lines.clone().len();
    let mut multipliers: Vec<u32> = vec!(1; num_cards);
    let sum: u32 = lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            let mut game_map: HashSet<i32> = HashSet::new();
            let map = l
                .split(':')
                .skip(1)
                .next()
                .unwrap()
                .split([':', '|', ' '])
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|v| {
                    if game_map.insert(v.parse::<i32>().unwrap()) {
                        0
                    } else {
                        1
                    }
                });

            let dups: u32 = map.sum();
            for j in 1..usize::try_from(dups+1).unwrap() {
                multipliers[i+j] = multipliers[i+j] + multipliers[i]
            }
            multipliers[i]
        })
        .sum();

    println!("part 2 ans: {}", sum);
    assert_eq!(sum, 5095824);
}
