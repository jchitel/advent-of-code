const INPUT: &'static str = include_str!("problem_1.txt");

pub fn part_1() -> usize {
    let mut sum = 0;
    for line in INPUT.lines() {
        let mut numbers = vec![];
        for char in line.chars() {
            if let Some(digit) = char.to_digit(10) {
                numbers.push(digit as usize);
                continue;
            }
        }

        let first = numbers.first().unwrap_or(&0);
        let last = numbers.last().unwrap_or(&0);
        sum += first * 10 + last;
    }
    return sum;
}

pub fn part_2() -> usize {
    let number_strs: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];

    let mut sum = 0;
    for line in INPUT.lines() {
        let mut numbers = vec![];
        for (idx, char) in line.char_indices() {
            if let Some(digit) = char.to_digit(10) {
                numbers.push(digit as usize);
                continue;
            }
            for (index, number_str) in number_strs.iter().enumerate() {
                if line.get(idx..idx+number_str.len()) == Some(number_str) {
                    numbers.push(index + 1);
                    break;
                }
            }
        }

        let first = numbers.first().unwrap_or(&0);
        let last = numbers.last().unwrap_or(&0);
        sum += first * 10 + last;
    }
    return sum;
}