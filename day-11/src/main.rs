fn main() {
    // part 1
    let lines: Vec<_> = include_str!("../input")
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect();
    
    let mut first_transform: Vec<String> = vec![];

    lines.iter().enumerate().for_each(|(i, l)| {
        let mut append_extra_line = true;
        l.chars().enumerate().for_each(|(j, c)| {
            if c != '.' { append_extra_line = false}
        });
        first_transform.push(l.to_string());
        if append_extra_line {first_transform.push(l.to_string())};
    });

    println!("{:#?}", first_transform);

    let mut indexes_to_add: Vec<bool> = vec![true; first_transform.clone().first().unwrap().len()];
    first_transform.clone().iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c != '.' { indexes_to_add[j] = false}
        });
    });
    let second_transform = first_transform.iter().enumerate().map(|(i, l)| {
        let mut s = String::new();
        l.chars().enumerate().for_each(|(j, c)| {
            s.push(c);
            if indexes_to_add[j] == true {s.push(c)}
        });
        s
    });

    let second_transform = second_transform.clone();

    let mut galaxies : Vec<(i32, i32)>= vec![];
    second_transform.clone().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c == '#' { galaxies.push((i as i32, j as i32))}
        });
    });
    
    println!("{:#?}", galaxies);

    let mut ans: usize = 0;
    while !galaxies.is_empty() {
        let (x_1, y_1) = galaxies.pop().unwrap();
        ans += galaxies.iter().map(|(x_2, y_2)| ((x_2-x_1).abs() + (y_2-y_1).abs()) as usize ).sum::<usize>()
    }

    println!("part 1 ans: {}", ans);
    assert_eq!(ans, 9550717);
}
