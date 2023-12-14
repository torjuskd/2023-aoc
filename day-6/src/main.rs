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
    let mut lines = lines.iter();
    let line_one = lines.next().unwrap();
    let times = line_one
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|d| d.parse::<u32>().unwrap());

    let line_two = lines.next().unwrap();
    let distances = line_two
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|d| d.parse::<u32>().unwrap());

    let sum: usize = times
        .zip(distances)
        .map(|(t, d)| {
            (0..t)
                .map(move |h| h * (t - h))
                .filter(move |dist| dist > &d)
                .collect::<Vec<_>>()
                .len()
        })
        .product();

    println!("part 1 ans: {}", sum);
    assert_eq!(sum, 1108800);

    // part 2
    let lines = read_lines("input");
    let mut lines = lines.iter().map(|s| s.replace(" ", ""));
    let line_one = lines.next().unwrap();
    let times = line_one
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|d| d.parse::<u64>().unwrap());

    let line_two = lines.next().unwrap();
    let distances = line_two
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|d| d.parse::<u64>().unwrap());

    let sum: usize = times
        .zip(distances)
        .map(|(t, d)| {
            (14..t)
                .map(move |h| h * (t - h))
                .filter(move |dist| dist > &d)
                .collect::<Vec<_>>()
                .len()
        })
        .product();

    println!("part 2 ans: {}", sum);
    assert_eq!(sum, 71503);
}
