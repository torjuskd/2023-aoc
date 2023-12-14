use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    // part 1
    let lines: Vec<_> = include_str!("../input")
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect();

    let perms: u64 = lines
        .iter()
        .map(|l| {
            let (springs, numbers) = l.split_once(' ').unwrap();
            let numbers = numbers
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect();

            let mut hm: HashMap<String, u64> = HashMap::new();
            recurse(springs.to_string(), numbers, &mut hm)
        })
        .sum();

    println!("part 1 ans: {}", perms);
    assert_eq!(perms, 7163);

    // part 2
    let lines: Vec<_> = include_str!("../input")
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect();

    let perms: u64 = lines
        .iter()
        .map(|l| {
            let (springs, numbers) = l.split_once(' ').unwrap();
            let springs = std::iter::once(springs).cycle().take(5).join("?");
            let numbers = numbers
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec();
            let numbers_len = numbers.len();
            let numbers = numbers
                .into_iter()
                .cycle()
                .take(5 * numbers_len)
                .collect_vec();

            let mut hm: HashMap<String, u64> = HashMap::new();
            recurse(springs, numbers, &mut hm)
        })
        .sum();

    println!("part 2 ans: {}", perms);
    assert_eq!(perms, 17788038834112);
}

fn recurse(s: String, numbers: Vec<u64>, hm: &mut HashMap<String, u64>) -> u64 {
    let k = &format!("{s},{}", numbers.iter().join(","));
    match hm.get(k) {
        Some(v) => return *v,
        None => (),
    }
    let ans = if s.is_empty() && numbers.is_empty() {
        1
    } else if numbers.is_empty() && !s.contains("#") {
        1
    } else if numbers.is_empty() && s.contains("#") {
        0
    } else if s.starts_with('.') {
        recurse(s[1..].to_string(), numbers, hm)
    } else if s.starts_with('?') {
        recurse("#".to_owned() + &s[1..].to_string(), numbers.clone(), hm)
            + recurse(".".to_owned() + &s[1..].to_string(), numbers, hm)
    } else if s.starts_with('#')
        && s.len() >= numbers[0] as usize
        && !s[..(numbers[0] as usize)].contains(".")
    {
        if s.len() > numbers[0] as usize && s[(numbers[0] as usize)..].starts_with('#') {
            0
        } else if s.len() > numbers[0] as usize {
            recurse(
                ".".to_owned() + &s[(numbers[0] as usize) + 1..].to_string(),
                numbers.split_at(1).1.to_vec(),
                hm,
            )
        } else {
            recurse(
                s[(numbers[0] as usize)..].to_string(),
                numbers.split_at(1).1.to_vec(),
                hm,
            )
        }
    } else {
        0
    };
    hm.insert(k.to_string(), ans);
    ans
}
