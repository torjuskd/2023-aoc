use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

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

    let line_length = lines.clone().first().unwrap().len();
    let number_of_lines = lines.len();
    let mut sum = 0;

    let mut vec = vec![vec!['.'; line_length + 2]; number_of_lines + 2];

    lines.iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            vec[i + 1][j + 1] = c;
        })
    });

    lines.iter().enumerate().for_each(|(i, l)| {
        let mut number = String::new();
        let mut should_add = false;
        l.chars().enumerate().for_each(|(j, c)| {
            if c.is_digit(10) {
                number.push(c);

                let to_check = [
                    vec[i+0][j+0],vec[i+0][j+1],vec[i+0][j+2],
                    vec[i+1][j+0],vec[i+1][j+1],vec[i+1][j+2],
                    vec[i+2][j+0],vec[i+2][j+1],vec[i+2][j+2],
                ];
                if to_check.iter().any(|c| !c.is_digit(10) && *c != '.') {
                    should_add = true
                };
                if !vec[i + 1][j + 2].is_digit(10) {
                    if should_add {
                        sum += number.parse::<i32>().unwrap();
                        number = String::new();
                        should_add = false;
                    } else {
                        number = String::new();
                        should_add = false;
                    }
                }
            } else {
                number = String::new()
            }
        })
    });

    println!("part 1 ans: {}", sum);
    assert_eq!(sum, 520019);

    // part 2
    let lines = read_lines("input");

    let line_length = lines.clone().first().unwrap().len();
    let number_of_lines = lines.len();
    let mut sum = 0;

    let mut vec = vec![vec!['.'; line_length + 2]; number_of_lines + 2];

    lines.iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            vec[i + 1][j + 1] = c;
        })
    });

    // en map fra gir-lokasjon til nummeret som allerede er lagt til,
    // hvis eksister når lagrer, multipliser verdiene
    let mut map: HashMap<String, i32> = HashMap::new();
    lines.iter().enumerate().for_each(|(i, l)| {
        let mut number = String::new();
        let mut should_add = false;
        let mut star_locations: HashSet<String> = HashSet::new();
        l.chars().enumerate().for_each(|(j, c)| {
            if c.is_digit(10) {
                number.push(c);

                let to_check = [
                    (vec[i + 0][j + 0], format!("{},{}", i + 0, j + 0)),
                    (vec[i + 0][j + 1], format!("{},{}", i + 0, j + 1)),
                    (vec[i + 0][j + 2], format!("{},{}", i + 0, j + 2)),
                    (vec[i + 1][j + 0], format!("{},{}", i + 1, j + 0)),
                    (vec[i + 1][j + 1], format!("{},{}", i + 1, j + 1)),
                    (vec[i + 1][j + 2], format!("{},{}", i + 1, j + 2)),
                    (vec[i + 2][j + 0], format!("{},{}", i + 2, j + 0)),
                    (vec[i + 2][j + 1], format!("{},{}", i + 2, j + 1)),
                    (vec[i + 2][j + 2], format!("{},{}", i + 2, j + 2)),
                ];
                to_check
                    .iter()
                    .filter(|(c, _s)| *c == '*')
                    .for_each(|(_c, s)| {
                        // må ta vare på indeksene til *alle* funnede '*' (legge til i set?)
                        star_locations.insert(s.to_string());
                        should_add = true
                    });
                if !vec[i + 1][j + 2].is_digit(10) {
                    if should_add {
                        let number_to_add = number.parse::<i32>().unwrap();
                        star_locations.iter().for_each(|location| {
                            let existing = map.insert(location.to_string(), number_to_add);

                            let multiplier = match existing {
                                Some(v) => v,
                                None => 0,
                            };
                            sum += multiplier * number_to_add;
                        });
                        number = String::new();
                        should_add = false;
                        star_locations = HashSet::new();
                    } else {
                        number = String::new();
                        should_add = false;
                        star_locations = HashSet::new();
                    }
                }
            } else {
                number = String::new()
            }
        })
    });

    println!("part 2 ans: {}", sum);
    assert_eq!(sum, 75519888);
}
