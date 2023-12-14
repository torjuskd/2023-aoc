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
    let lines = lines.iter();

    let sum_next_hist_vals: i64 = lines
        .map(|l| {
            l.split(' ')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|l| {
            let mut diffs: Vec<Vec<i64>> = vec![];
            let mut nums = l;
            let first_row = nums.clone();
            diffs.push(first_row);

            while !nums.iter().all(|n| n == &0) {
                let nums_1 = nums.iter().rev().skip(1).rev();
                let nums_2 = nums.iter().skip(1);
                nums = nums_1.zip(nums_2).map(|(a, b)| b - a).collect();
                diffs.push(nums.clone());
            }
            diffs.iter().map(|d| d.last().unwrap()).sum::<i64>()
        })
        .sum();

    println!("part 1 ans: {}", sum_next_hist_vals);
    assert_eq!(sum_next_hist_vals, 2005352194);

    // part 2
    let lines = read_lines("input");
    let lines = lines.iter();

    let sum_prev_hist_vals: i64 = lines
        .map(|l| {
            l.split(' ')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|l| {
            let mut diffs: Vec<Vec<i64>> = vec![];
            let mut rev  = l.clone();
            rev.reverse();
            let mut nums = rev;
            let first_row = nums.clone();
            diffs.push(first_row);

            while !nums.iter().all(|n| n == &0) {
                let nums_1 = nums.iter().rev().skip(1).rev();
                let nums_2 = nums.iter().skip(1);
                nums = nums_1.zip(nums_2).map(|(a, b)| b - a).collect();
                diffs.push(nums.clone());
            }
            println!("{:#?}",diffs);
            diffs.iter().map(|d| d.last().unwrap()).sum::<i64>()
        })
        .sum();

    println!("part 2 ans: {}", sum_prev_hist_vals);
    assert_eq!(sum_prev_hist_vals, 1077);
}
