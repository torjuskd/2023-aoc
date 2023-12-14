fn main() {
    // part 1
    let lines: Vec<_> = include_str!("../input")
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect();
        
    let mut i_indexes_to_add: Vec<bool> = vec![true; lines.clone().len()];
    let mut j_indexes_to_add: Vec<bool> = vec![true; lines.clone().first().unwrap().len()];
    let mut galaxies: Vec<(i64, i64)> = vec![];

    lines.iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                galaxies.push((i as i64, j as i64));
                j_indexes_to_add[j] = false;
                i_indexes_to_add[i] = false;
            }
        });
    });

    let to_add = 1;
    let mut ans = 0;
    while !galaxies.is_empty() {
        let (x_1, y_1) = galaxies.pop().unwrap();
        ans += galaxies
            .iter()
            .map(|(x_2, y_2)| -> i64 {

                let x_1 = expand_x(x_1, &i_indexes_to_add, to_add);
                let x_2 = expand_x(*x_2, &i_indexes_to_add, to_add);

                let y_1 = expand_y(y_1, &j_indexes_to_add, to_add);
                let y_2 = expand_y(*y_2, &j_indexes_to_add, to_add);


                let temp = (x_2 - x_1).abs() + (y_2 - y_1).abs();
                    
                        temp
            })
            .sum::<i64>()
    }

    println!("part 1 ans: {}", ans);
    assert_eq!(ans, 9550717);

    // part 2
    let lines: Vec<_> = include_str!("../input")
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect();

    let mut i_indexes_to_add: Vec<bool> = vec![true; lines.clone().len()];
    let mut j_indexes_to_add: Vec<bool> = vec![true; lines.clone().first().unwrap().len()];
    let mut galaxies: Vec<(i64, i64)> = vec![];

    lines.iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                galaxies.push((i as i64, j as i64));
                j_indexes_to_add[j] = false;
                i_indexes_to_add[i] = false;
            }
        });
    });

    let to_add = 1000000-1;
    let mut ans = 0;
    while !galaxies.is_empty() {
        let (x_1, y_1) = galaxies.pop().unwrap();
        ans += galaxies
            .iter()
            .map(|(x_2, y_2)| -> i64 {

                let x_1 = expand_x(x_1, &i_indexes_to_add, to_add);
                let x_2 = expand_x(*x_2, &i_indexes_to_add, to_add);

                let y_1 = expand_y(y_1, &j_indexes_to_add, to_add);
                let y_2 = expand_y(*y_2, &j_indexes_to_add, to_add);


                let temp = (x_2 - x_1).abs() + (y_2 - y_1).abs();
                    
                        temp
            })
            .sum::<i64>()
    }

    println!("part 2 ans: {}", ans);
    assert_eq!(ans, 648458253817);
}

fn expand_y(y: i64, j_indexes_to_add: &Vec<bool>, to_add: i64) -> i64 {
    y +
    (0..y)
            .map(|y| {
                if j_indexes_to_add[y as usize] {
                    to_add
                } else {
                    0
                }
            })
            .sum::<i64>()
}

fn expand_x(x: i64, i_indexes_to_add: &Vec<bool>, to_add: i64) -> i64 {
    x +
    (0..x)
            .map(|x| {
                if i_indexes_to_add[x as usize] {
                    to_add
                } else {
                    0
                }
            }).sum::<i64>()
}
