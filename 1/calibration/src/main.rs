use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut numbers = HashMap::new();
    numbers.insert("one", '1');
    numbers.insert("two", '2');
    numbers.insert("three", '3');
    numbers.insert("four", '4');
    numbers.insert("five", '5');
    numbers.insert("six", '6');
    numbers.insert("seven", '7');
    numbers.insert("eight", '8');
    numbers.insert("nine", '9');

    let sum: u32 = contents
        .lines()
        .map(|line| {
            let mut first = ' ';
            let mut last = ' ';
            let mut map = HashMap::new();

            for (idx, char) in line.char_indices() {
                for i in 0..idx + 1 {
                    let value: &String = match map.entry(i) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => v.insert(String::new()),
                    };

                    let value = format!("{}{}", value, char);
                    if numbers.contains_key(&value.as_str()) {
                        let v = *numbers.get(&value.as_str()).unwrap();
                        if first == ' ' {
                            first = v;
                            last = v;
                        } else {
                            last = v;
                        }
                    }

                    map.insert(i, value);
                }

                if char.is_numeric() {
                    if first == ' ' {
                        first = char;
                        last = char;
                    } else {
                        last = char;
                    }
                }
            }

            let num = format!("{}{}", first, last).parse::<u32>().unwrap_or(0);
            return num;
        })
        .sum();

    println!("Sum: {}", sum)
}
