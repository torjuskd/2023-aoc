fn main() {
    // part 1
    let lines: Vec<_> = include_str!("../input")
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect();

    let perms: i32 = lines
        .iter()
        .map(|l| {
            let mut springs_numbers = l.split(' ');
            let springs = springs_numbers.next().unwrap();
            let numbers = springs_numbers
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            let n = recurse(springs.to_string(), numbers);
            //println!("{n}");
            n
        })
        .sum();

    let ans = perms;
    println!("part 1 ans: {}", ans);
    assert_eq!(ans, 7163);

    // part 2
    let lines: Vec<_> = include_str!("../test_input")
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect();

    let perms: i32 = lines
        .iter()
        .map(|l| {
            let mut springs_numbers = l.split(' ');
            let mut springs = springs_numbers.next().unwrap();
            let f = format!("{}?{}?{}?{}?{}", &springs, &springs, &springs, &springs, &springs);
            springs = &f;
            let numbers = springs_numbers
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let numbers = (vec![numbers; 5]).iter().flatten().map(|n| n.clone()).collect::<Vec<_>>();
            let n = recurse(springs.to_string(), numbers);
            n
        })
        .sum();

    let ans = perms;
    println!("part 2 ans: {}", ans);
    assert_eq!(ans, 525152);
}

fn recurse(s: String, numbers: Vec<i32>) -> i32 {
    if s.is_empty() && numbers.is_empty() {
        1
    } else if numbers.is_empty() && !s.contains("#") {
        1
    } else if numbers.is_empty() && s.contains("#") {
        0
    } else if s.starts_with('.') {
        recurse(s[1..].to_string(), numbers)
    } else if s.starts_with('?') {
        recurse("#".to_owned() + &s[1..].to_string(), numbers.clone())
            + recurse(".".to_owned() + &s[1..].to_string(), numbers)
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
            )
        } else {
            recurse(
                s[(numbers[0] as usize)..].to_string(),
                numbers.split_at(1).1.to_vec(),
            )
        }
    } else {
        0
    }
}
