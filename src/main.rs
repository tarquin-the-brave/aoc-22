mod cli;
mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    use cli::{Cli, Day, Part};
    let cli: Cli = clap::Parser::parse();
    let input_file_path = cli.input_file_path();
    let input_str = std::fs::read_to_string(&input_file_path).expect(&format!(
        "Failed to open input file {}",
        input_file_path.to_string_lossy()
    ));

    match (cli.day, cli.part) {
        (Day::One, Part::One) => {
            println!("{}", day1::part1(input_str));
        }
        (Day::One, Part::Two) => {
            println!("{}", day1::part2(input_str));
        }
        (Day::Two, Part::One) => {
            println!("{}", day2::part1(input_str));
        }
        (Day::Two, Part::Two) => {
            println!("{}", day2::part2(input_str));
        }
        (Day::Three, Part::One) => {
            println!("{}", day3::part1(input_str));
        }
        (Day::Three, Part::Two) => {
            println!("{}", day3::part2(input_str));
        }
        (Day::Four, Part::One) => {
            println!("{}", day4::part1(input_str));
        }
        (Day::Four, Part::Two) => {
            println!("{}", day4::part2(input_str));
        }
        (Day::Five, Part::One) => {
            println!("{}", day5::part1(input_str));
        }
        (Day::Five, Part::Two) => {
            println!("{}", day5::part2(input_str));
        }
        (Day::Six, Part::One) => {
            println!("{}", day6::part1(input_str));
        }
        (Day::Six, Part::Two) => {
            println!("{}", day6::part2(input_str));
        }
        (Day::Seven, Part::One) => {
            println!("{}", day7::part1(input_str));
        }
        (Day::Seven, Part::Two) => {
            println!("{}", day7::part2(input_str));
        }
        (Day::Eight, Part::One) => {
            println!("{}", day8::part1(input_str));
        }
        (Day::Eight, Part::Two) => {
            println!("{}", day8::part2(input_str));
        }
        (Day::Nine, Part::One) => {
            println!("{}", day9::part1(input_str.clone()));
            println!("{}", day9::part1_refactored(input_str));
        }
        (Day::Nine, Part::Two) => {
            println!("{}", day9::part2(input_str));
        }
        (Day::Ten, Part::One) => {
            println!("{}", day10::part1(input_str));
        }
        (Day::Ten, Part::Two) => {
            println!("{}", day10::part2(input_str));
        }
    }
}
