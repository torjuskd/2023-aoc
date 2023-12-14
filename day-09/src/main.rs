fn main() {
    // part 1
    let input_nums = read_input_nums();
    let sum_next_hist_vals: i64 = sum_hist_vals(input_nums);

    println!("part 1 ans: {}", sum_next_hist_vals);
    assert_eq!(sum_next_hist_vals, 2005352194);

    // part 2
    let input_nums = read_input_nums();
    let input_nums = input_nums
        .iter()
        .map(|l| l.iter().rev().map(|n| n.clone()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let sum_prev_hist_vals: i64 = sum_hist_vals(input_nums);

    println!("part 2 ans: {}", sum_prev_hist_vals);
    assert_eq!(sum_prev_hist_vals, 1077);
}

fn read_input_nums() -> Vec<Vec<i64>> {
    let lines: Vec<_> = include_str!("../input")
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect();
    let lines = lines.iter();

    let input_nums = lines
        .map(|l| {
            l.split(' ')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();
    input_nums
}

fn sum_hist_vals(input_nums: Vec<Vec<i64>>) -> i64 {
    input_nums
        .iter()
        .map(|l| {
            let mut diffs: Vec<Vec<i64>> = vec![];
            let mut nums = l.clone();
            let first_row = nums.clone();
            diffs.push(first_row);

            while !nums.iter().all(|n| n == &0) {
                let nums_1 = nums.iter().rev().skip(1).rev();
                let nums_2 = nums.iter().skip(1);
                nums = nums_1.zip(nums_2).map(|(a, b)| (b - a).clone()).collect();
                diffs.push(nums.clone());
            }
            diffs.iter().map(|d| d.last().unwrap()).sum::<i64>()
        })
        .sum()
}
