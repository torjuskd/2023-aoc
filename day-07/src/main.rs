use std::{cmp::Ordering, fs::read_to_string};

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
    let lines = lines.iter();
    // from high card to five of a kind
    let mut types: Vec<Vec<(&str, u32)>> = vec![vec![]; 7];

    let cards_bids = lines.map(|l| {
        let mut split = l.split(' ');
        let first = split.next().unwrap();
        let second = split.next().unwrap().parse::<u32>().unwrap();
        (first, second)
    });

    let possible_cards = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    cards_bids.for_each(|(card, bid)| {
        let matches = possible_cards.map(|possible_card| card.match_indices(possible_card).count());
        if matches.contains(&5) {
            types[6].push((card, bid));
        } else if matches.contains(&4) {
            types[5].push((card, bid));
        } else if matches.contains(&3) && matches.contains(&2) {
            types[4].push((card, bid));
        } else if matches.contains(&3) {
            types[3].push((card, bid));
        } else if matches.iter().filter(|count| **count == 2).count() == 2 {
            types[2].push((card, bid));
        } else if matches.contains(&2) {
            types[1].push((card, bid));
        } else {
            types[0].push((card, bid))
        }
    });
    let sorted = types
        .iter_mut()
        .map(|t| {
            t.sort_by(|a, b| {
                let (a_s, _a_b) = a;
                let (b_s, _b_b) = b;

                a_s.chars()
                    .map(|char_to_find_index_of| {
                        possible_cards
                            .iter()
                            .position(|c| c == &char_to_find_index_of)
                            .unwrap()
                    })
                    .zip(b_s.chars().map(|char_to_find_index_of| {
                        possible_cards
                            .iter()
                            .position(|c| c == &char_to_find_index_of)
                            .unwrap()
                    }))
                    .map(|(a, b)| a.cmp(&b))
                    .find(|r| r == &Ordering::Greater || r == &Ordering::Less)
                    .or(Some(Ordering::Equal))
                    .unwrap()
            });
            t
        })
        .flatten();

    let sum = sorted
        .map(|(_str, bid)| bid)
        .enumerate()
        .map(|(i, bid)| usize::try_from(*bid).unwrap() * (i + 1))
        .sum::<usize>();
    println!("part 1 ans: {}", sum);
    assert_eq!(sum, 255048101);
}
