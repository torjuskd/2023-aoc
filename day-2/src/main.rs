//use regex::Regex;
use std::collections::HashMap;
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
    let mut sum = 0;

    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let game_map = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    lines.iter().for_each(|l| {
        let mut split = l.split(":");
        let game_number = split
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let rounds = split.next().unwrap().split(";");
        let mut should_add = true;

        rounds.for_each(|round| {
            let color_amounts = round.trim().split(", ");
            color_amounts.for_each(|color_amount| {
                let mut split = color_amount.split(" ");
                let amount = split.next().unwrap().parse::<i32>().unwrap();
                let color = split.next().unwrap();

                let limit_surpassed = match game_map.get(color) {
                    Some(limit) => limit < &amount,
                    None => false,
                };
                if limit_surpassed {
                    should_add = false
                };
            });
        });
        if should_add {
            sum += game_number
        };
    });

    println!("part 1 list: {:#?}", lines);
    println!("part 1 ans: {}", sum);

   // let mut sum = 0;
   // let global_map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

   // let game_maps = lines.iter().map(|l| {
   //     let mut split = l.split(":");
   //     let _game_number = split
   //         .next()
   //         .unwrap()
   //         .split(" ")
   //         .last()
   //         .unwrap()
   //         .parse::<i32>()
   //         .unwrap();
   //     let rounds = split.next().unwrap().split(";");
   //     let mut game_map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

   //     rounds.for_each(|round| {
   //         let color_amounts = round.trim().split(", ");
   //         color_amounts.for_each(|color_amount| {
   //             let mut split = color_amount.split(" ");
   //             let amount = split.next().unwrap().parse::<i32>().unwrap();
   //             let color = split.next().unwrap();

   //             let update = match game_map.get(color) {
   //                 Some(prev_max) => prev_max < &amount,
   //                 None => false,
   //             };

   //             if update { game_map.insert(color, amount); }
   //         });
   //     });
   //     game_map
   // });

   // game_maps.map(|m| m.vala )
}
