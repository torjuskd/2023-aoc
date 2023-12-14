fn main() {
    // part 1
    let lines: Vec<_> = include_str!("../input")
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect();
    let line_length = lines.clone().first().unwrap().len();
    let number_of_lines = lines.len();

    let mut vec = vec![vec!['.'; line_length + 2]; number_of_lines + 2];
    let mut steps = vec![vec![usize::MAX; line_length + 2]; number_of_lines + 2];
    let mut motions: Vec<Vec<Vec<(usize, usize)>>> =
        vec![vec![vec![]; line_length + 2]; number_of_lines + 2];

    let mut start: (usize, usize) = (0, 0);
    lines.iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            let x = i + 1;
            let y = j + 1;

            let north = (i + 0, j + 1);
            let west = (i + 1, j + 0);
            let east = (i + 1, j + 2);
            let south = (i + 2, j + 1);

            vec[x][y] = c;
            motions[x][y] = match c {
                '|' => vec![north, south],
                '-' => vec![east, west],
                'L' => vec![north, east],
                'J' => vec![north, west],
                '7' => vec![south, west],
                'F' => vec![south, east],
                'S' => vec![north, west, south, east],
                _ => vec![],
            };

            if c == 'S' {
                start = (x, y);
            }
        })
    });

    traverse(start.0, start.1, 0, &mut steps, &motions);

    println!("{:#?}", steps);

    let ans = steps
        .iter()
        .map(|s| s.iter().filter(|s| *s != &usize::MAX))
        .flatten()
        .max()
        .unwrap();

    println!("part 1 ans: {}", ans);
    assert_eq!(ans, &6733);
}

fn traverse(
    x: usize,
    y: usize,
    nsteps: usize,
    steps: &mut Vec<Vec<usize>>,
    motions: &Vec<Vec<Vec<(usize, usize)>>>,
) {
    let prev_val = steps[x][y];

    println!("{x}, {y}, steps: {nsteps}");
    if prev_val == usize::MAX {
        steps[x][y] = nsteps;
    } else {
        if prev_val > nsteps {
            steps[x][y] = nsteps;
        } else {
            return;
        }
    }

    motions[x][y].iter().for_each(|(x_2, y_2)| {
        let t = &motions[*x_2][*y_2];
        if t.iter().any(|i| i.0 == x && i.1 == y) {
            traverse(*x_2, *y_2, nsteps + 1, steps, &motions);
        }
    });
}
