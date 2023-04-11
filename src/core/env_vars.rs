pub fn env_var(key: &str) -> String {
    dotenv::var(key).expect(&format!("{} is not set", key))
}
