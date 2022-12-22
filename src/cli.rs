#[derive(clap::ValueEnum, Clone)]
pub enum Day {
    #[value(name = "1")]
    One,
    #[value(name = "2")]
    Two,
    #[value(name = "3")]
    Three,
    #[value(name = "4")]
    Four,
    #[value(name = "5")]
    Five,
    #[value(name = "6")]
    Six,
    #[value(name = "7")]
    Seven,
    #[value(name = "8")]
    Eight,
    #[value(name = "9")]
    Nine,
    #[value(name = "10")]
    Ten,
    #[value(name = "11")]
    Eleven,
    #[value(name = "12")]
    Twelve,
    #[value(name = "13")]
    Thirteen,
    #[value(name = "14")]
    Fourteen,
    #[value(name = "15")]
    Fifteen,
    #[value(name = "16")]
    Sixteen,
    #[value(name = "17")]
    Seventeen,
    #[value(name = "18")]
    Eighteen,
    #[value(name = "19")]
    Nineteen,
    #[value(name = "20")]
    Twenty,
    #[value(name = "21")]
    TwentyOne,
    #[value(name = "22")]
    TwentyTwo,
    #[value(name = "23")]
    TwentyThree,
    #[value(name = "24")]
    TwentyFour,
    #[value(name = "25")]
    TwentyFive,
}

#[derive(clap::ValueEnum, Clone)]
pub enum Part {
    #[value(name = "1")]
    One,
    #[value(name = "2")]
    Two,
}

#[derive(clap::ValueEnum, Clone)]
pub enum Input {
    Test,
    Full,
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Day::One => write!(f, "1"),
            Day::Two => write!(f, "2"),
            Day::Three => write!(f, "3"),
            Day::Four => write!(f, "4"),
            Day::Five => write!(f, "5"),
            Day::Six => write!(f, "6"),
            Day::Seven => write!(f, "7"),
            Day::Eight => write!(f, "8"),
            Day::Nine => write!(f, "9"),
            Day::Ten => write!(f, "10"),
            Day::Eleven => write!(f, "11"),
            Day::Twelve => write!(f, "12"),
            Day::Thirteen => write!(f, "13"),
            Day::Fourteen => write!(f, "14"),
            Day::Fifteen => write!(f, "15"),
            Day::Sixteen => write!(f, "16"),
            Day::Seventeen => write!(f, "17"),
            Day::Eighteen => write!(f, "18"),
            Day::Nineteen => write!(f, "19"),
            Day::Twenty => write!(f, "20"),
            Day::TwentyOne => write!(f, "21"),
            Day::TwentyTwo => write!(f, "22"),
            Day::TwentyThree => write!(f, "23"),
            Day::TwentyFour => write!(f, "24"),
            Day::TwentyFive => write!(f, "25"),
        }
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Input::Test => write!(f, "test"),
            Input::Full => write!(f, "full"),
        }
    }
}

#[derive(clap::Parser)]
// #[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_enum)]
    pub day: Day,
    #[arg(short, long, value_enum)]
    pub part: Part,
    #[arg(short, long, value_enum)]
    pub input: Input,
    #[arg(long, default_value = "inputs")]
    input_dir: std::path::PathBuf,
}

impl Cli {
    pub fn input_file_path(&self) -> std::path::PathBuf {
        let mut path_buf = self.input_dir.to_path_buf();
        path_buf.push(format!("day{}-{}.txt", self.day, self.input));
        path_buf
    }
}
