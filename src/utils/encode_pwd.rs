use argon2::{self, Config};

pub fn encode_pwd(password: &String) -> String {
    let pwd = password.as_bytes();
    let salt = b"ger654g1r5CX%zfeATY_AHFn583!_fa";
    let config = Config::default();

    argon2::hash_encoded(pwd, salt, &config).unwrap()
}
