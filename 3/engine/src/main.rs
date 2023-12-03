use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();

    let lines_vec: Vec<Vec<char>> = contents
        .lines()
        .map(|l| {
            let mut v: Vec<char> = Vec::new();
            for c in l.chars() {
                v.push(c);
            }
            v
        })
        .collect();

    // first(&lines_vec);
    second(&lines_vec);
}

fn first(lines_vec: &Vec<Vec<char>>) {
    println!("FIRST");
    let mut adjacent_numbers: Vec<u32> = Vec::new();
    for (line_idx, line) in lines_vec.iter().enumerate() {
        let mut number = String::new();
        let mut is_adjacent = false;
        for (char_idx, c) in line.iter().enumerate() {
            if is_sign(*c) && !number.is_empty() {
                adjacent_numbers.push(number.parse().unwrap());
                number = String::new();
                is_adjacent = false;
            } else if c.is_numeric() {
                number = format!("{}{}", number, *c);

                if char_idx > 0 && is_sign(line[char_idx - 1]) {
                    is_adjacent = true;
                }

                if line_idx > 0 {
                    if char_idx > 0 && is_sign(lines_vec[line_idx - 1][char_idx - 1]) {
                        is_adjacent = true;
                    }
                    if is_sign(lines_vec[line_idx - 1][char_idx]) {
                        is_adjacent = true;
                    }
                    if char_idx < line.len() - 1 && is_sign(lines_vec[line_idx - 1][char_idx + 1]) {
                        is_adjacent = true;
                    }
                }

                if line_idx < lines_vec.len() - 1 {
                    if char_idx > 0 && is_sign(lines_vec[line_idx + 1][char_idx - 1]) {
                        is_adjacent = true
                    }
                    if is_sign(lines_vec[line_idx + 1][char_idx]) {
                        is_adjacent = true;
                    }
                    if char_idx < line.len() - 1 && is_sign(lines_vec[line_idx + 1][char_idx + 1]) {
                        is_adjacent = true;
                    }
                }

                if char_idx == line.len() - 1 && is_adjacent {
                    adjacent_numbers.push(number.parse().unwrap());
                    number = String::new();
                    is_adjacent = false;
                }
            } else {
                if !number.is_empty() && is_adjacent {
                    adjacent_numbers.push(number.parse().unwrap())
                }
                number = String::new();
                is_adjacent = false;
            }
        }
    }

    println!("sum: {}", adjacent_numbers.iter().sum::<u32>())
}

fn second(lines_vec: &Vec<Vec<char>>) {
    let mut adjacent_numbers: HashMap<String, Vec<u32>> = HashMap::new();
    for (line_idx, line) in lines_vec.iter().enumerate() {
        let mut number = String::new();
        let mut adjacent_asterixs: HashMap<String, bool> = HashMap::new();
        for (char_idx, c) in line.iter().enumerate() {
            if is_asterix(*c) && !number.is_empty() {
                let idx = format!("{}{}", line_idx, char_idx);
                adjacent_asterixs.insert(idx, true);

                for key in adjacent_asterixs.keys() {
                    let n = number.parse().unwrap();
                    adjacent_numbers
                        .entry(key.clone())
                        .and_modify(|v| v.push(n))
                        .or_insert(vec![n]);
                }

                number = String::new();
                adjacent_asterixs = HashMap::new();
            } else if c.is_numeric() {
                number = format!("{}{}", number, *c);

                if char_idx > 0 && is_asterix(line[char_idx - 1]) {
                    let idx = format!("{}{}", line_idx, char_idx - 1);
                    adjacent_asterixs.insert(idx, true);
                }

                if line_idx > 0 {
                    if char_idx > 0 && is_asterix(lines_vec[line_idx - 1][char_idx - 1]) {
                        let idx = format!("{}{}", line_idx - 1, char_idx - 1);
                        adjacent_asterixs.insert(idx, true);
                    }
                    if is_asterix(lines_vec[line_idx - 1][char_idx]) {
                        let idx = format!("{}{}", line_idx - 1, char_idx);
                        adjacent_asterixs.insert(idx, true);
                    }
                    if char_idx < line.len() - 1
                        && is_asterix(lines_vec[line_idx - 1][char_idx + 1])
                    {
                        let idx = format!("{}{}", line_idx - 1, char_idx + 1);
                        adjacent_asterixs.insert(idx, true);
                    }
                }

                if line_idx < lines_vec.len() - 1 {
                    if char_idx > 0 && is_asterix(lines_vec[line_idx + 1][char_idx - 1]) {
                        let idx = format!("{}{}", line_idx + 1, char_idx - 1);
                        adjacent_asterixs.insert(idx, true);
                    }
                    if is_asterix(lines_vec[line_idx + 1][char_idx]) {
                        let idx = format!("{}{}", line_idx + 1, char_idx);
                        adjacent_asterixs.insert(idx, true);
                    }
                    if char_idx < line.len() - 1
                        && is_asterix(lines_vec[line_idx + 1][char_idx + 1])
                    {
                        let idx = format!("{}{}", line_idx + 1, char_idx + 1);
                        adjacent_asterixs.insert(idx, true);
                    }
                }

                if char_idx == line.len() - 1 {
                    for key in adjacent_asterixs.keys() {
                        let n = number.parse().unwrap();
                        adjacent_numbers
                            .entry(key.clone())
                            .and_modify(|v| v.push(n))
                            .or_insert(vec![n]);
                    }

                    number = String::new();
                    adjacent_asterixs = HashMap::new();
                }
            } else {
                for key in adjacent_asterixs.keys() {
                    let n = number.parse().unwrap();
                    adjacent_numbers
                        .entry(key.clone())
                        .and_modify(|v| v.push(n))
                        .or_insert(vec![n]);
                }

                number = String::new();
                adjacent_asterixs = HashMap::new();
            }
        }
    }

    let sum: u32 = adjacent_numbers
        .values()
        .filter(|a| a.len() == 2)
        .map(|e| e.iter().product::<u32>())
        .sum();

    println!("sum: {}", sum);
}

fn is_asterix(c: char) -> bool {
    c == '*'
}

fn is_sign(c: char) -> bool {
    if c.is_numeric() {
        return false;
    }
    if c == '.' {
        return false;
    }
    true
}
