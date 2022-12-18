#[derive(clap::ValueEnum, Clone)]
pub enum Day {
    #[value(name = "1")]
    One,
    #[value(name = "2")]
    Two,
    #[value(name = "3")]
    Three,
}

#[derive(clap::ValueEnum, Clone)]
pub enum Part {
    #[value(name = "1")]
    One,
    #[value(name = "2")]
    Two,
}

#[derive(clap::ValueEnum, Clone)]
enum Input {
    Example,
    Test,
    Full,
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Day::One => write!(f, "1"),
            Day::Two => write!(f, "2"),
            Day::Three => write!(f, "3"),
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
            Input::Example => write!(f, "example"),
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
    input: Input,
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
