use crate::config::Config;

pub fn generate_password(config: &Config) -> String {
    let mut set: Vec<char> = Vec::new();

    if config.letters.enabled {
        set.extend(config.letters.chars.trim().chars());
    }

    for charset in &config.charsets {
        set.extend(charset.chars.trim().chars());
    }
    
    let mut res = String::new();
    if set.is_empty() {
        return res
    }
    for _ in 0..config.len {
        let mut char = set.get(fastrand::usize(0..set.len())).unwrap_or(&'\0').to_string();
        if config.letters.use_capitals && fastrand::bool() {
            char = char.to_uppercase();
        }
        res += &char;
    }
    res
}
