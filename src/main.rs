use clap::Parser;

mod aoc_2023;

fn main() {
    let args = Args::parse();

    match (args.year, args.problem, args.part) {
        (2023, 1, Some(1)) => println!("{}", aoc_2023::problem_1::part_1()),
        (2023, 1, Some(2)) => println!("{}", aoc_2023::problem_1::part_2()),
        (2023, 1, None) => println!("Part 1: {}\nPart 2: {}", aoc_2023::problem_1::part_1(), aoc_2023::problem_1::part_2()),
        (2023, 2, Some(1)) => println!("{}", aoc_2023::problem_2::part_1()),
        (2023, 2, Some(2)) => println!("{}", aoc_2023::problem_2::part_2()),
        (2023, 2, None) => println!("Part 1: {}\nPart 2: {}", aoc_2023::problem_2::part_1(), aoc_2023::problem_2::part_2()),
        (year, _, _) if year < 2015 || year > 2023 => panic!("Invalid year {}", year),
        (year, problem, _) if problem < 1 || problem > 25 => panic!("Invalid problem {} for year {}", problem, year),
        _ => panic!("Problem {} from year {} is not yet solved!", args.problem, args.year),
    }
}

#[derive(Parser)]
struct Args {
    year: u16,
    problem: u8,
    part: Option<u8>,
}