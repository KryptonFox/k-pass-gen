use crate::KPassGenApp;
use std::ops::AddAssign;

pub fn generate_password_to_ctx(app: &mut KPassGenApp) {
    app.password = generate_password(app.config.len, &app.config.charset);
}

pub fn generate_password(len: usize, charset: &String) -> String {
    let mut res = String::new();
    let set = charset.split("").collect::<Vec<&str>>();
    for _ in 0..len {
        res.add_assign(set.get(fastrand::usize(0..set.len())).unwrap_or(&""));
    }
    res
}
