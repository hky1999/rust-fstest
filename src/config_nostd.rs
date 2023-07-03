pub struct Config {
    pub rounds: u32,
    pub bytes: u64,
    pub verbose: bool,
}

impl Config {
    pub fn parse() -> Self {
        Config {
            rounds: 100,
            bytes: 4096,
            verbose: true,
        }
    }
    pub fn rounds_and_bytes(&self) -> (u32, u64) {
        (self.rounds, self.bytes)
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}