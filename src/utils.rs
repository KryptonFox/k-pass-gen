use crate::KPassGenApp;

pub fn generate_password_to_ctx(app: &mut KPassGenApp) {
    app.password = generate_password(app.config.len, &app.config.charset, app.config.use_capitals);
}

pub fn generate_password(len: usize, charset: &String, use_capitals: bool) -> String {
    let mut res = String::new();
    let set = charset.split("").collect::<Vec<&str>>();
    for _ in 0..len {
        let mut char = set.get(fastrand::usize(0..set.len())).unwrap_or(&"").to_string();
        if use_capitals && fastrand::bool() {
            char = char.to_uppercase();
        }
        res += &char;
    }
    res
}
