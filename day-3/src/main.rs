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

    let line_length = lines.clone().first().unwrap().len();
    let number_of_lines = lines.len();
    let mut sum = 0;

    let mut vec = vec![vec!['.'; line_length+2]; number_of_lines+2];

    lines.iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            vec[i+1][j+1] = c;
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
                if to_check.iter().any(|c| !c.is_digit(10) && *c != '.') {should_add = true};
                if !vec[i+1][j+2].is_digit(10) {
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

}
