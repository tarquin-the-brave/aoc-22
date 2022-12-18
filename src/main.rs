mod cli;
mod day1;
mod day2;
mod day3;

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
        _ => unimplemented!(),
    }
}
