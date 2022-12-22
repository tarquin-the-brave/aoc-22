mod cli;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    use cli::{Cli, Day, Input, Part};
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
        (Day::Eleven, Part::One) => {
            println!("{}", day11::part1(input_str));
        }
        (Day::Eleven, Part::Two) => {
            println!("{}", day11::part2(input_str));
        }
        (Day::Twelve, Part::One) => {
            println!("{}", day12::part1(input_str));
        }
        (Day::Twelve, Part::Two) => {
            println!("{}", day12::part2(input_str));
        }
        (Day::Thirteen, Part::One) => {
            println!("{}", day13::part1(input_str));
        }
        (Day::Thirteen, Part::Two) => {
            println!("{}", day13::part2(input_str));
        }
        (Day::Fourteen, Part::One) => {
            println!("{}", day14::part1(input_str));
        }
        (Day::Fourteen, Part::Two) => {
            println!("{}", day14::part2(input_str));
        }
        (Day::Fifteen, Part::One) => {
            println!(
                "{}",
                day15::part1(
                    input_str,
                    match cli.input {
                        Input::Test => 10,
                        Input::Full => 2_000_000,
                    }
                )
            );
        }
        (Day::Fifteen, Part::Two) => {
            println!("{}", day15::part2(input_str));
        }
        (Day::Sixteen, Part::One) => {
            println!("{}", day16::part1(input_str));
        }
        (Day::Sixteen, Part::Two) => {
            println!("{}", day16::part2(input_str));
        }
        (Day::Seventeen, Part::One) => {
            println!("{}", day17::part1(input_str));
        }
        (Day::Seventeen, Part::Two) => {
            println!("{}", day17::part2(input_str));
        }
        (Day::Eighteen, Part::One) => {
            println!("{}", day18::part1(input_str));
        }
        (Day::Eighteen, Part::Two) => {
            println!("{}", day18::part2(input_str));
        }
        (Day::Nineteen, Part::One) => {
            println!("{}", day19::part1(input_str));
        }
        (Day::Nineteen, Part::Two) => {
            println!("{}", day19::part2(input_str));
        }
        (Day::Twenty, Part::One) => {
            println!("{}", day20::part1(input_str));
        }
        (Day::Twenty, Part::Two) => {
            println!("{}", day20::part2(input_str));
        }
        (Day::TwentyOne, Part::One) => {
            println!("{}", day21::part1(input_str));
        }
        (Day::TwentyOne, Part::Two) => {
            println!("{}", day21::part2(input_str));
        }
        (Day::TwentyTwo, Part::One) => {
            println!("{}", day22::part1(input_str));
        }
        (Day::TwentyTwo, Part::Two) => {
            println!("{}", day22::part2(input_str));
        }
        (Day::TwentyThree, Part::One) => {
            println!("{}", day23::part1(input_str));
        }
        (Day::TwentyThree, Part::Two) => {
            println!("{}", day23::part2(input_str));
        }
        (Day::TwentyFour, Part::One) => {
            println!("{}", day24::part1(input_str));
        }
        (Day::TwentyFour, Part::Two) => {
            println!("{}", day24::part2(input_str));
        }
        (Day::TwentyFive, Part::One) => {
            println!("{}", day25::part1(input_str));
        }
        (Day::TwentyFive, Part::Two) => {
            println!("{}", day25::part2(input_str));
        }
    }
}
