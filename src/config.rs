use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Config {
    #[arg(
        long = "rounds",
        short = 'r',
        default_value = "10",
        help = "number of rounds the test will run"
    )]
    pub rounds: u32,
    #[arg(
        long = "bytes",
        short = 'b',
        default_value = "4096",
        help = "bytes to read or write"
    )]
    pub bytes: u64,
    #[arg(
        long = "verbose",
        short = 'v',
        default_value_t = false,
        help = "verbosity value"
    )]
    pub verbose: bool,
}

impl Config {
    pub fn rounds_and_bytes(&self) -> (u32, u64) {
        (self.rounds, self.bytes)
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}
