fn main() {
    // common
    let lines: Vec<_> = include_str!("../input")
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect();
    let mut x_is_to_exp: Vec<bool> = vec![true; lines.clone().len()];
    let mut y_is_to_exp: Vec<bool> = vec![true; lines.clone().first().unwrap().len()];
    let mut galaxies: Vec<(i64, i64)> = vec![];
    lines.iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                galaxies.push((i as i64, j as i64));
                y_is_to_exp[j] = false;
                x_is_to_exp[i] = false;
            }
        });
    });

    // part 1
    let ans = calc_sum_of_dists(&galaxies, &x_is_to_exp, 1, &y_is_to_exp);
    println!("part 1 ans: {}", ans);
    assert_eq!(ans, 9550717);

    // part 2
    let ans = calc_sum_of_dists(&galaxies, &x_is_to_exp, 1000000 - 1, &y_is_to_exp);
    println!("part 2 ans: {}", ans);
    assert_eq!(ans, 648458253817);
}

fn calc_sum_of_dists(
    galaxies: &Vec<(i64, i64)>,
    x_indicies_to_expand: &Vec<bool>,
    expansion_factor: i64,
    y_indicies_to_expand: &Vec<bool>,
) -> i64 {
    let mut galaxies = galaxies.clone();
    let mut ans = 0;
    while !galaxies.is_empty() {
        let (x_1, y_1) = galaxies.pop().unwrap();
        ans += galaxies
            .iter()
            .map(|(x_2, y_2)| -> i64 {
                let x_1 = expand(x_1, &x_indicies_to_expand, expansion_factor);
                let x_2 = expand(*x_2, &x_indicies_to_expand, expansion_factor);
                let y_1 = expand(y_1, &y_indicies_to_expand, expansion_factor);
                let y_2 = expand(*y_2, &y_indicies_to_expand, expansion_factor);
                (x_2 - x_1).abs() + (y_2 - y_1).abs()
            })
            .sum::<i64>()
    }
    ans
}

fn expand(x_or_y: i64, x_or_y_indicies_to_expand: &Vec<bool>, expansion_factor: i64) -> i64 {
    x_or_y
        + ((0..x_or_y)
            .filter(|y| x_or_y_indicies_to_expand[*y as usize])
            .count() as i64)
            * expansion_factor
}
