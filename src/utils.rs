use crate::config::Config;
use crate::KPassGenApp;

pub fn generate_password_to_ctx(app: &mut KPassGenApp) {
    app.password = generate_password(&app.config);
}

pub fn generate_password(config: &Config) -> String {
    let mut set: Vec<char> = Vec::new();

    if config.letters.enabled {
        set.extend(config.letters.chars.chars());
    }
    
    for charset in &config.charsets {
        set.extend(charset.chars.chars());
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
