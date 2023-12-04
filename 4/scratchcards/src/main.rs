use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();

    first(&contents);
    second(&contents);
}

fn first(contents: &str) {
    let res: u32 = contents
        .lines()
        .map(|l| {
            let game_split: Vec<&str> = l.split(':').collect();
            let number_split: Vec<&str> = game_split[1].split('|').collect();

            let winning_numbers: Vec<i32> = number_split[0]
                .trim()
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            let numbers: Vec<i32> = number_split[1]
                .trim()
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            let all_winning: Vec<i32> = numbers
                .iter()
                .map(|n| {
                    if winning_numbers.contains(n) {
                        return 1;
                    }

                    0
                })
                .filter(|i| *i > 0)
                .collect();

            if all_winning.is_empty() {
                0
            } else {
                2_u32.pow((all_winning.len() - 1).try_into().unwrap())
            }
        })
        .sum();

    println!("first: {}", res);
}

fn second(contents: &str) {
    let scratchcards_org: Vec<&str> = contents.lines().collect();
    let mut scratchcards: Vec<&str> = contents.lines().collect();

    let mut card_idx = 0;
    while card_idx < scratchcards.len() {
        let card = scratchcards[card_idx];
        let game_split: Vec<&str> = card.split(':').collect();
        let game_num: Vec<&str> = game_split[0].split_whitespace().collect();
        let game_num: u32 = game_num[1].parse::<u32>().unwrap();

        let number_split: Vec<&str> = game_split[1].split('|').collect();
        let winning_numbers: Vec<i32> = number_split[0]
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let numbers: Vec<i32> = number_split[1]
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let all_winning: Vec<i32> = numbers
            .iter()
            .map(|n| {
                if winning_numbers.contains(n) {
                    return 1;
                }

                0
            })
            .filter(|i| *i > 0)
            .collect();

        let mut i = 1;
        for _ in all_winning {
            // -1 because in _org its represented with indexes
            let win_card_idx: usize = ((game_num - 1) + i).try_into().unwrap();
            if win_card_idx < scratchcards_org.len() {
                scratchcards.push(scratchcards_org[win_card_idx])
            }

            i += 1;
        }

        card_idx += 1;
    }

    println!("second: {}", scratchcards.len());
}
