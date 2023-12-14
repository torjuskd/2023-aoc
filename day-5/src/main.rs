use std::{fs::read_to_string, ops::ControlFlow};

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
    let seeds = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap());
    let lines = lines.skip(2);
    let mut maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut map: Vec<(u64, u64, u64)> = Vec::new();

    lines.for_each(|l| {
        if l.len() == 0 {
            maps.push(map.clone());
            map = Vec::new();
        } else if l.chars().any(|c| c.is_digit(10)) {
            let mut split = l.split(' ');
            let tuple = (
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
            );
            map.push(tuple);
        }
    });
    // add last map at end of file
    maps.push(map.clone());

    let locs = seeds.map(|seed_number| {
        let mut cur_number = seed_number;
        maps.iter().for_each(|m| {
            m.iter().try_for_each(|(dest_start, src_start, range_len)| {
                if src_start <= &cur_number && cur_number < (src_start + range_len) {
                    cur_number = dest_start + (cur_number - src_start);
                    return ControlFlow::Break(cur_number);
                }
                ControlFlow::Continue(())
            });
        });
        cur_number
    });
    let lowest_loc_num = locs.min().unwrap();

    println!("part 1 ans: {}", lowest_loc_num);
    assert_eq!(lowest_loc_num, 199602917);
}
