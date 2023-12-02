use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    second(contents);
}

fn second(contents: String) {
    let mut sum_vec: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let mut max_map = HashMap::new();
        max_map.insert("blue", 0);
        max_map.insert("green", 0);
        max_map.insert("red", 0);

        let game_split: Vec<&str> = line.split(':').collect();
        let sack = game_split[1];

        let tries_split: Vec<&str> = sack.split(';').collect();

        for hand in tries_split {
            let hand_split: Vec<&str> = hand.split(',').collect();
            for marble in hand_split {
                let m = marble.trim();

                let quantity_split: Vec<&str> = m.split(' ').collect();
                let color = quantity_split[1];
                let num: u32 = quantity_split[0].parse().unwrap();

                max_map.entry(color).and_modify(|v| {
                    if num > *v {
                        *v = num
                    }
                });
            }
        }

        sum_vec.push(max_map.values().product());
    }

    println!("sum: {}", sum_vec.iter().sum::<u32>());
}

fn first(contents: String) {
    let mut max = HashMap::new();
    max.insert("blue", 14);
    max.insert("green", 13);
    max.insert("red", 12);

    let mut sum_vec: Vec<u32> = Vec::new();

    'line: for line in contents.lines() {
        let game_split: Vec<&str> = line.split(':').collect();
        let game = game_split[0];

        let num_split: Vec<&str> = game.split(' ').collect();
        let game_number: u32 = num_split[1].parse().unwrap();

        let sack = game_split[1];

        let tries_split: Vec<&str> = sack.split(';').collect();

        for hand in tries_split {
            let hand_split: Vec<&str> = hand.split(',').collect();
            for marble in hand_split {
                let m = marble.trim();

                let quantity_split: Vec<&str> = m.split(' ').collect();
                let num: u32 = quantity_split[0].parse().unwrap();
                let max = *max.get(quantity_split[1]).unwrap();

                if num > max {
                    continue 'line;
                }
            }
        }

        sum_vec.push(game_number);
    }

    println!("sum: {}", sum_vec.iter().sum::<u32>());
}
