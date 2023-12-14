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

                if game_map.get(color).unwrap() < &amount {
                    should_add = false;
                };
            });
        });
        if should_add {
            sum += game_number
        };
    });

    println!("part 1 ans: {}", sum);
    assert_eq!(sum, 2593);

    // part 2
    let game_maps = lines.iter().map(|l| {
        let mut split = l.split(":");
        let _game_number = split
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let rounds = split.next().unwrap().split(";");
        let mut game_map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        rounds.for_each(|round| {
            let color_amounts = round.trim().split(", ");
            color_amounts.for_each(|color_amount| {
                let mut split = color_amount.split(" ");
                let amount = split.next().unwrap().parse::<i32>().unwrap();
                let color = split.next().unwrap();

                if game_map.get(color).unwrap() < &amount {
                    game_map.insert(color, amount);
                };
            });
        });
        game_map.values().product::<i32>()
    });

    let sum: i32 = game_maps.sum();
    println!("part 2 ans: {}", sum);
    assert_eq!(sum, 54699);
}
