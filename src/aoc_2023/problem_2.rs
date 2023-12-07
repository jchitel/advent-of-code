use regex::Regex;

const INPUT: &'static str = include_str!("problem_2.txt");

pub fn part_1() -> usize {
    let r_game = Regex::new(r"^Game (\d+): (.+)$").unwrap();
    let r_color = Regex::new(r"(\d+) (red|blue|green)").unwrap();

    let mut sum = 0;
    for line in INPUT.lines() {
        let (_, [game_num, sets]) = r_game.captures(line).unwrap().extract();
        let game_num = game_num.parse::<usize>().unwrap();
        let mut valid = true;
        for set in sets.split("; ") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for color in set.split(", ") {
                let (_, [count, color]) = r_color.captures(color).unwrap().extract();
                let count = count.parse::<usize>().unwrap();
                match color {
                    "red" => red = count,
                    "green" => green = count,
                    "blue" => blue = count,
                    _ => panic!("Invalid input: {}", line),
                }
            }
            if red > 12 || green > 13 || blue > 14 {
                valid = false;
                break;
            }
        }
        if valid {
            sum += game_num;
        }
    }
    return sum;
}

pub fn part_2() -> usize {
    let r_game = Regex::new(r"^Game (\d+): (.+)$").unwrap();
    let r_color = Regex::new(r"(\d+) (red|blue|green)").unwrap();

    let mut sum = 0;
    for line in INPUT.lines() {
        let (_, [_, sets]) = r_game.captures(line).unwrap().extract();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for set in sets.split("; ") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for color in set.split(", ") {
                let (_, [count, color]) = r_color.captures(color).unwrap().extract();
                let count = count.parse::<usize>().unwrap();
                match color {
                    "red" => red = count,
                    "green" => green = count,
                    "blue" => blue = count,
                    _ => panic!("Invalid input: {}", line),
                }
            }
            if red > max_red { max_red = red; }
            if green > max_green { max_green = green; }
            if blue > max_blue { max_blue = blue; }
        }
        sum += max_red * max_green * max_blue;
    }
    return sum;
}
