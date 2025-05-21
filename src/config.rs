pub struct Config {
    pub len: usize,
    pub charset: String,
    pub use_capitals: bool,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            len: 8,
            charset: "abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
            use_capitals: true,
        }
    }
}